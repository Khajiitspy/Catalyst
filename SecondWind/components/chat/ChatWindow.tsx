import { FC, useEffect, useRef, useState } from "react";
import {
    View,
    Text,
    ScrollView,
    TouchableOpacity,
} from "react-native";

import { InputField } from "@/components/form/InputField";
import { useForm } from "@/hooks/useForm";

import { getChatConnection } from "@/hubs/chatHub";
import {
    useGetChatMessagesQuery,
    useAmIAdminQuery,
} from "@/services/chatService";
import { IMessageItem } from "@/types/chat/IMessageItem";

import EditChatModal from "./EditChatModal";

interface ChatWindowProps {
    chatId: number | null;
}

const ChatWindow: FC<ChatWindowProps> = ({ chatId }) => {
    const scrollRef = useRef<ScrollView>(null);

    const { data: history } = useGetChatMessagesQuery(chatId ?? 0, {
        skip: !chatId,
    });

    const { data: isAdmin } = useAmIAdminQuery(chatId ?? 0, {
        skip: !chatId,
    });

    const [messages, setMessages] = useState<IMessageItem[]>([]);
    const [editVisible, setEditVisible] = useState(false);

    const msgForm = useForm<{ message: string }>({ message: "" });

    useEffect(() => {
        if (history) setMessages(history);
    }, [history]);

    useEffect(() => {
        if (!chatId) return;

        const connection = getChatConnection();
        if (!connection) return;

        connection.invoke("JoinChat", chatId);

        const handler = (msg: IMessageItem) =>
            setMessages(prev => [...prev, msg]);

        connection.on("ReceiveMessage", handler);

        return () => {
            connection.invoke("LeaveChat", chatId);
            connection.off("ReceiveMessage", handler);
            setMessages([]);
        };
    }, [chatId]);

    const sendMessage = () => {
        const text = msgForm.form.message.trim();
        if (!text || !chatId) return;

        const connection = getChatConnection();
        if (!connection) return;

        connection.invoke("SendMessage", { chatId, message: text });

        msgForm.setForm({ message: "" });
    };

    if (!chatId) {
        return (
            <View className="flex-1 items-center justify-center">
                <Text className="text-zinc-400">Оберіть чат</Text>
            </View>
        );
    }

    return (
        <View className="flex-1">

            <View className="flex-row items-center justify-between p-3 border-b border-zinc-300 dark:border-zinc-700">
                <Text className="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
                    Чат
                </Text>

                {isAdmin && (
                    <TouchableOpacity
                        onPress={() => setEditVisible(true)}
                        className="px-3 py-1 bg-emerald-500 rounded-lg"
                    >
                        <Text className="text-white font-semibold">
                            Редагувати
                        </Text>
                    </TouchableOpacity>
                )}
            </View>

            <ScrollView
                ref={scrollRef}
                className="flex-1 p-4"
                contentContainerStyle={{ gap: 8 }}
                onContentSizeChange={() =>
                    scrollRef.current?.scrollToEnd({ animated: true })
                }
            >
                {messages.map((m, i) => (
                    <View
                        key={m.id ?? i}
                        className="bg-zinc-200 dark:bg-zinc-800 p-3 rounded-xl self-start max-w-[85%]"
                    >
                        <Text className="text-zinc-600 dark:text-zinc-400 font-semibold mb-1">
                            {m.userName || "Інший користувач"}
                        </Text>
                        <Text className="text-zinc-900 dark:text-zinc-100">
                            {m.message}
                        </Text>
                    </View>
                ))}
            </ScrollView>

            <View className="flex-row p-2 border-t border-zinc-300 dark:border-zinc-700 items-end gap-2">
                <View className="flex-1">
                    <InputField
                        placeholder="Напишіть повідомлення..."
                        value={msgForm.form.message}
                        onChangeText={msgForm.onChange("message")}
                        onSubmitEditing={sendMessage}
                    />
                </View>

                <TouchableOpacity
                    onPress={sendMessage}
                    className="bg-emerald-500 px-4 py-3 rounded-xl"
                >
                    <Text className="text-white font-semibold">OK</Text>
                </TouchableOpacity>
            </View>

            <EditChatModal
                chatId={chatId}
                visible={editVisible}
                onClose={() => setEditVisible(false)}
            />
        </View>
    );
};

export default ChatWindow;
