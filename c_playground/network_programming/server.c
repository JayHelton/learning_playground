#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <unistd.h>
#include <errno.h>
#include <stdlib.h>

#define ISVALIDSOCKET(s) ((s) >= 0)
#define CLOSESOCKET(s) close(s)
#define SOCKET int
#define GETSOCKETERRNO() (errno)

#include <stdio.h>
#include <string.h>
#include <time.h>

int main() {
    struct addrinfo hints;
    memset(&hints, 0, sizeof(hints));

    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_flags = AI_PASSIVE;

    struct addrinfo *bind_address;
    getaddrinfo(0, "8080", &hints, &bind_address);

    /**
     * Create a socket listener using the info from getadrinfo
     * Easier to use getadrinfo instead of manual parameters
     */
    SOCKET socket_listen;
    socket_listen = socket(bind_address->ai_family,
            bind_address->ai_socktype, bind_address->ai_protocol);

    if (!ISVALIDSOCKET(socket_listen)) {
        fprintf(stderr, "socket() failed. (%d)\n", GETSOCKETERRNO());
        return 1;
    }

    /**
     * Bind to an address and IPV4
     */
    if (bind(socket_listen,
                bind_address->ai_addr, bind_address->ai_addrlen)) {
        fprintf(stderr, "bind() failed. (%d)\n", GETSOCKETERRNO());
        return 1;
    }

    /**
     * Free up memory from the data we no longer need
     */
    freeaddrinfo(bind_address);

    if (listen(socket_listen, 10) < 0) {
        fprintf(stderr, "listen() failed. (%d)\n", GETSOCKETERRNO());
        return 1;
    }

    /**
     * Create an infinite loop to make sure the server
     * keeps accepting new connections
     */
    while(1) {
        /**
         * Start accepting connections that come through
         */
        struct sockaddr_storage client_address;
        socklen_t client_len = sizeof(client_address);
        SOCKET socket_client = accept(socket_listen,
                (struct sockaddr*) &client_address, &client_len);

        if (!ISVALIDSOCKET(socket_client)) {
            fprintf(stderr, "accept() failed. (%d)\n", GETSOCKETERRNO());
            return 1;
        }

        /**
         * accept() pauses execution until a client is connected
         * 
         * Once connected, get address info from the client
         */
        char address_buffer[100];
        getnameinfo((struct sockaddr*)&client_address,
                client_len, address_buffer, sizeof(address_buffer), 0, 0,
                NI_NUMERICHOST);

        /**
         * Fork allows a process to split off to a new new process per connection
         */
        int pid = fork();

        if (pid == 0) {
            CLOSESOCKET(socket_listen);
            /**
             * Reading the bytes from the request
             */
            char request[1024];
            int bytes_received = recv(socket_client, request, 1024, 0);

            /**
             * This is a dumb server, so just return the response
             *
             * Format the response bytes in HTTP format
             */
            const char *response =
                "HTTP/1.1 200 OK\r\n"
                "Connection: close\r\n"
                "Content-Type: text/plain\r\n\r\n"
                "Local time is: ";

            /**
             * Send the initial bytes to the client
             */
            int bytes_sent = send(socket_client, response, strlen(response), 0);
            printf("Sent %d of %d bytes.\n", bytes_sent, (int)strlen(response));

            /**
             * Create the data for the timestamp
             *
             * The server can continue to send data while the socket is open.
             * The client will consider everything as part of the http request
             * until the socket conneciton is closed by the server or it timesout
             */
            time_t timer;
            time(&timer);

            char *time_msg = ctime(&timer);
            send(socket_client, time_msg, strlen(time_msg), 0);

            CLOSESOCKET(socket_client);
            exit(0);
        }
        CLOSESOCKET(socket_client);
    }

    CLOSESOCKET(socket_listen);

    return 0;
}

