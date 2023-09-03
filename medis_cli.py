import socket

host = '127.0.0.1'
port = 9876

if __name__ == '__main__':
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((host, port))

    while True:
        message = input('medist_cli> ')
        if message == 'quit':
            break

        s.send(message.encode())
        
        response = s.recv(1024)  
        print(response.decode())  

    s.close()
