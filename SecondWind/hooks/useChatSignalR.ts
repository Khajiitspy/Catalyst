import { useEffect, useState, useRef, useCallback } from 'react';
import { HubConnectionBuilder, HubConnection, LogLevel } from '@microsoft/signalr';
import { IChatMessage } from "@/types/сhat/IChatMessage";
import { APP_URLS } from "@/constants/Urls";

export const useChatSignalR = (chatId: number, token?: string) => {
    const [messages, setMessages] = useState<IChatMessage[]>([]);
    const [isConnected, setIsConnected] = useState(false);
    const connectionRef = useRef<HubConnection | null>(null);

    useEffect(() => {
        if (!token) return;

        const startConnection = async () => {
            try {
                const hubUrl = `${APP_URLS.BASE_URL}/hubs/chat`;
                const connection = new HubConnectionBuilder()
                    .withUrl(hubUrl, { accessTokenFactory: () => token })
                    .withAutomaticReconnect()
                    .configureLogging(LogLevel.Information)
                    .build();

                connection.on("ReceiveMessage", (msg: IChatMessage) => {
                    setMessages(prev => [msg, ...prev]);
                });

                await connection.start();
                setIsConnected(true);

                await connection.invoke("JoinChat", chatId);
                connectionRef.current = connection;
            } catch (err) {
                console.error("❌ Connection failed:", err);
                setIsConnected(false);
            }
        };

        startConnection();

        return () => {
            if (connectionRef.current) {
                connectionRef.current.invoke("LeaveChat", chatId).catch(() => {});
                connectionRef.current.stop();
                connectionRef.current = null;
                setIsConnected(false);
            }
        };
    }, [chatId, token]);

    const sendMessage = useCallback(async (text: string) => {
        if (!text.trim() || !connectionRef.current) return;
        try {
            await connectionRef.current.invoke("SendMessage", { chatId, message: text });
            return true;
        } catch (err) {
            console.error("❌ Send failed:", err);
            return false;
        }
    }, [chatId]);

    return {
        messages,
        setMessages,
        sendMessage,
        isConnected
    };
};
