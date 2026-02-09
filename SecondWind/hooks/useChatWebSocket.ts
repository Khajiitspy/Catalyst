import { useEffect, useRef, useState, useCallback } from "react";
import { IChatMessage } from "@/types/сhat/IChatMessage";
import { APP_URLS } from "@/constants/Urls";

export const useChatWebSocket = (chatId: number, token?: string) => {
    const [messages, setMessages] = useState<IChatMessage[]>([]);
    const [isConnected, setIsConnected] = useState(false);
    const socketRef = useRef<WebSocket | null>(null);

    useEffect(() => {
        if (!token || !chatId) return;

        const wsUrl = APP_URLS.BASE_URL
            .replace("http://", "ws://")
            .replace("https://", "wss://") + "/hubs/chat";

        const socket = new WebSocket(wsUrl, [], {
            headers: {
                Authorization: `Bearer ${token}`,
            },
        });

        socketRef.current = socket;

        socket.onopen = () => {
            setIsConnected(true);

            socket.send(JSON.stringify({
                type: "JoinChat",
                chat_id: chatId,
            }));
        };

        socket.onmessage = (event) => {
            try {
                const msg: IChatMessage = JSON.parse(event.data);
                setMessages(prev => [msg, ...prev]);
            } catch (err) {
                console.warn("Invalid WS message", err);
            }
        };

        socket.onerror = (err) => {
            console.error("❌ WS error", err);
        };

        socket.onclose = () => {
            setIsConnected(false);
        };

        return () => {
            socket.send(JSON.stringify({
                type: "LeaveChat",
                chat_id: chatId,
            }));
            socket.close();
        };
    }, [chatId, token]);

    const sendMessage = useCallback((text: string) => {
        if (!text.trim() || !socketRef.current || socketRef.current.readyState !== 1) {
            return false;
        }

        socketRef.current.send(JSON.stringify({
            type: "SendMessage",
            chat_id: chatId,
            message: text,
        }));

        return true;
    }, [chatId]);

    return {
        messages,
        setMessages,
        sendMessage,
        isConnected,
    };
};
