import { useEffect, useRef, useState, useCallback } from "react";
import { IChatMessage } from "@/types/сhat/IChatMessage";
import { BASE_URL } from "@/constants/Urls";
import { useAppSelector } from "@/store";

export const useChatWebSocket = (chatId: number | null, token?: string) => {
    const socketRef = useRef<WebSocket | null>(null);
    const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);
    const [messages, setMessages] = useState<IChatMessage[]>([]);
    const [isConnected, setIsConnected] = useState(false);
    const { user } = useAppSelector((state) => state.auth);

    const connect = useCallback(() => {
        if (!chatId || !token) return;

        const wsUrl =
            BASE_URL.replace("http://", "ws://").replace("https://", "wss://") +
            `/hubs/chat?token=${token}`;

        const socket = new WebSocket(wsUrl);
        socketRef.current = socket;

        socket.onopen = () => {
            setIsConnected(true);
            try {
                socket.send(JSON.stringify({ type: "JoinChat", chat_id: chatId }));
            } catch (err) {
                console.warn("WS send failed on join:", err);
            }
        };

        socket.onmessage = (event) => {
            try {
                const msg = JSON.parse(event.data);

                setMessages(prev => [
                    ...prev,
                    {
                        ...msg,
                        isMine: msg.userId === user?.id,
                    },
                ]);
            } catch (err) {
                console.warn("Invalid WS message", err);
            }
        };

        socket.onerror = (err) => {
            console.error("WebSocket error:", err);
        };

        socket.onclose = () => {
            setIsConnected(false);
            if (reconnectTimeoutRef.current) clearTimeout(reconnectTimeoutRef.current);
            reconnectTimeoutRef.current = setTimeout(() => {
                connect();
            }, 3000);
        };
    }, [chatId, token, user?.id]);

    useEffect(() => {
        connect();

        return () => {
            if (reconnectTimeoutRef.current) clearTimeout(reconnectTimeoutRef.current);

            const s = socketRef.current;
            if (!s) return;

            try {
                if (s.readyState === WebSocket.OPEN) {
                    s.send(JSON.stringify({ type: "LeaveChat", chat_id: chatId }));
                }
            } catch (err) {
                console.warn("WS send failed on cleanup:", err);
            }

            try {
                if (s.readyState !== WebSocket.CLOSED) s.close();
            } catch (err) {
                console.warn("WS close failed on cleanup:", err);
            }

            socketRef.current = null;
        };
    }, [connect]);

    // const sendMessage = (text: string) => {
    //     if (!socketRef.current || socketRef.current.readyState !== WebSocket.OPEN) return;

    //     // create optimistic message with tempId
    //     const tempId = `temp-${Date.now()}`;
    //     const optimisticMsg: IChatMessage = {
    //         tempId,
    //         id: undefined,
    //         message: text,
    //         userId: user?.id || 0,
    //         userName: user?.name || "You",
    //         userImage: user?.image || null,
    //         isMine: true,
    //         createdAt: new Date().toISOString(),
    //     };

    //     setMessages((prev) => [...prev, optimisticMsg]);

    //     try {
    //         socketRef.current.send(
    //             JSON.stringify({ type: "SendMessage", chat_id: chatId, message: text, tempId })
    //         );
    //     } catch (err) {
    //         console.warn("WS send failed:", err);
    //     }
    // };
    const sendMessage = (text: string) => {
        if (!socketRef.current || socketRef.current.readyState !== WebSocket.OPEN) return;

        socketRef.current.send(JSON.stringify({
            type: "SendMessage",
            chat_id: chatId,
            message: text,
        }));
    };

    return { messages, setMessages, sendMessage, isConnected };
};
