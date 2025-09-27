#include <sys/socket.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <linux/connector.h>
#include <linux/netlink.h>
#include <linux/cn_proc.h>

//https://nick-black.com/dankwiki/index.php/The_Proc_Connector_and_Socket_Filters


int netlink_connect();
int netlink_disconnect(int netlink_sock);
int netlink_subscribe(int netlink_sock, int idx, int val);
int get_proc_event(int netlink_sock, struct proc_event *event);

//int main(int argc, char **argv){
//	//====== establish netlink connection to kernel ======
//	int sockfd = netlink_connect(CN_IDX_PROC);
//	if (sockfd < 0) return 1;
//
//	//====== subscribe to proc events ======
//	if (netlink_subscribe(sockfd,CN_IDX_PROC,CN_VAL_PROC) < 0){
//		netlink_disconnect(sockfd);
//		return 1;
//	}
//	
//	//====== get events ======
//	for (;;){
//		struct proc_event proc_event = {0};
//		int result = get_proc_event(sockfd,&proc_event);
//		if (result < 0) break;
//		switch (proc_event.what){
//		case PROC_EVENT_EXEC:
//			printf("exec called on %lu\n",proc_event.event_data.exec.process_pid);
//		}
//	}
//
//	//====== cleanup ======
//	netlink_disconnect(sockfd);
//}
int netlink_connect(int groups){
	struct sockaddr_nl netlink_addr = {
		.nl_family = AF_NETLINK,
		.nl_groups = groups,
		.nl_pid = getpid(),
	};
	int netlink_sock = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);
	if (netlink_sock < 0){
		perror("socket");
		return -1;
	}
	if (bind(netlink_sock,(struct sockaddr *)&netlink_addr,sizeof(netlink_addr)) < 0){
		perror("bind");
		close(netlink_sock);
		return -1;
	}

	//add ourselves to a new group so we dont have to be root
	int opt = netlink_addr.nl_groups;
	if (setsockopt(netlink_sock,SOL_NETLINK,NETLINK_ADD_MEMBERSHIP,&opt,sizeof(opt)) < 0){
		perror("setsockopt");
		close(netlink_sock);
		return -1;
	}

	return netlink_sock;
}
int netlink_disconnect(int netlink_sock){
	return close(netlink_sock);
}
int netlink_subscribe(int netlink_sock, int idx, int val){
	//====== construct message to ask to subscribe ======
	//nesting a proc connector operator in a connector message in a netlink message
	struct __attribute__((aligned(NLMSG_ALIGNTO))) {//netlink message
		struct nlmsghdr netlink_header;
		struct __attribute__((__packed__)) {
			struct cn_msg connector_message; //connector message
			enum proc_cn_mcast_op connector_operator; //proc connector operator
		};
	} message;
	memset(&message,0,sizeof(message));
	//netlink headers
	message.netlink_header.nlmsg_len = sizeof(message);
	message.netlink_header.nlmsg_pid = getpid();
	message.netlink_header.nlmsg_pid = NLMSG_DONE;
	//connector headers
	message.connector_message.id.idx = idx;
	message.connector_message.id.val = val;
	message.connector_message.seq = 0;
	message.connector_message.ack = 0;
	message.connector_message.len = sizeof(message.connector_operator);
	//proc connector operator
	message.connector_operator = PROC_CN_MCAST_LISTEN;
	return write(netlink_sock,&message,sizeof(message));
}
int get_proc_event(int netlink_sock, struct proc_event *event){
	//====== receive some data ======
	for (;;){
		struct __attribute__((aligned(NLMSG_ALIGNTO))) {//netlink message
			struct nlmsghdr netlink_header;
			struct __attribute__((__packed__)) {
				struct cn_msg connector_message; //connector message
				struct proc_event proc_event;
			};
		} message;
		memset(&message,0,sizeof(message));
		//recvfrom data
		struct sockaddr_nl addr;
		socklen_t addrlen = sizeof(addr);
		long long result = recvfrom(netlink_sock,&message,sizeof(message),0,(struct sockaddr *)&addr,&addrlen);
		if (result == 0) return -1;
		//====== filter out data not from the kernel ======
		if(addr.nl_pid != 0) continue;
		//====== unpack data ======
		if (message.netlink_header.nlmsg_type == NLMSG_ERROR || message.netlink_header.nlmsg_type == NLMSG_NOOP) continue; //ignore error and no op
		if (message.connector_message.id.idx != CN_IDX_PROC || message.connector_message.id.val != CN_VAL_PROC) continue; //ignore non proc data

		memcpy(event,&message.proc_event,sizeof(struct proc_event));
		//switch (message.proc_event.what){
		//case PROC_EVENT_EXEC:
		//	printf("exec called on %lu\n",message.proc_event.event_data.exec.process_pid);
		//}
		break;
	}
	return 0;
}
