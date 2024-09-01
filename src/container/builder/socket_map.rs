use crate::container::socket::Socket;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketMap {
    pub outer_port: u16,
    pub inner_socket: Socket,
}

impl SocketMap {
    pub fn new(outer_port: u16, inner_socket: Socket) -> Self {
        Self {
            outer_port,
            inner_socket,
        }
    }

    pub fn self_bound(socket: Socket) -> Self {
        Self::new(socket.port, socket)
    }
}

impl Into<String> for SocketMap {
    fn into(self) -> String {
        format!(
            "{}:{}",
            self.outer_port,
            self.inner_socket.format_protocol()
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::container::socket::Socket;

    use super::SocketMap;

    #[test]
    fn constructor_test() {
        let map = SocketMap::new(20, Socket::new(455, Some("TCP".into())));
        assert_eq!(map.outer_port, 20);
    }

    #[test]
    fn selfbind_test() {
        let sock = SocketMap::self_bound(Socket::new(455, Some("TCP".into())));
        assert_eq!(sock.inner_socket.port, sock.outer_port)
    }
}
