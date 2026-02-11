import { FC, useEffect, useRef, useState } from "react";
import {
    View,
    Text,
    ScrollView,
    TouchableOpacity,
    Image,
} from "react-native";

import { InputField } from "@/components/form/InputField";
import { useForm } from "@/hooks/useForm";
import { useAppSelector } from "@/store";
import { useChatWebSocket } from "@/hooks/useChatWebSocket";
import {
    useGetChatMessagesQuery,
    useAmIAdminQuery,
} from "@/services/chatService";
import { IMessageItem } from "@/types/chat/IMessageItem";
import EditChatModal from "./EditChatModal";
import { IMAGE_URL } from "@/constants/Urls";

interface ChatWindowProps {
    chatId: number | null;
}

const ChatWindow: FC<ChatWindowProps> = ({ chatId }) => {
    const scrollRef = useRef<ScrollView>(null);
    const { user } = useAppSelector(s => s.auth);

    const { data: history, isFetching } =
        useGetChatMessagesQuery(chatId ?? 0, {
            skip: !chatId,
            refetchOnMountOrArgChange: true,
            refetchOnFocus: true,
            refetchOnReconnect: true,
            pollingInterval: 2000,
        });

    const { data: isAdmin } = useAmIAdminQuery(chatId ?? 0, {
        skip: !chatId,
    });

    const {
        messages,
        setMessages,
        sendMessage,
        isConnected,
    } = useChatWebSocket(chatId, user?.token);

    const [editVisible, setEditVisible] = useState(false);
    const msgForm = useForm<{ message: string }>({ message: "" });

    /** REST → initial messages */
    useEffect(() => {
        if (!history || !user) return;

        setMessages(
            history.map(m => ({
                ...m,
                isMine: m.userId === user.id,
            }))
        );
    }, [history, user]);

    const send = () => {
        const text = msgForm.form.message.trim();
        if (!text) return;

        sendMessage(text);
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
            {/* Header */}
            <View className="flex-row items-center justify-between p-3 border-b border-zinc-300 dark:border-zinc-700">
                <Text className="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
                    Чат {isFetching && "..."}
                </Text>

                {isAdmin && (
                    <TouchableOpacity
                        onPress={() => setEditVisible(true)}
                        className="px-3 py-1 bg-emerald-500 rounded-lg"
                    >
                        <Text className="text-white font-semibold">Редагувати</Text>
                    </TouchableOpacity>
                )}
            </View>

            {/* Messages */}
            <ScrollView
                ref={scrollRef}
                className="flex-1 p-4"
                contentContainerStyle={{ gap: 8, paddingBottom: 20 }}
                keyboardShouldPersistTaps="handled"
                onContentSizeChange={() =>
                    scrollRef.current?.scrollToEnd({ animated: true })
                }
            >
                {messages.map((m, i) => (
                    <View
                        key={m.id ?? `msg-${i}`}
                        className={`p-3 rounded-xl max-w-[85%] flex-row gap-2 ${
                            m.isMine
                                ? "self-end bg-emerald-500"
                                : "self-start bg-zinc-200 dark:bg-zinc-800"
                        }`}
                    >
                        {!m.isMine && (
                            <Image
                                source={{
                                    uri: m.userImage
                                        ? `${IMAGE_URL}100_${m.userImage}`
                                        : undefined,
                                }}
                                className="w-10 h-10 rounded-full"
                            />
                        )}

                        <View className="flex-1">
                            {!m.isMine && (
                                <Text className="text-zinc-600 dark:text-zinc-400 font-semibold mb-1">
                                    {m.userName || "Користувач"}
                                </Text>
                            )}

                            <Text
                                className={
                                    m.isMine
                                        ? "text-white"
                                        : "text-zinc-900 dark:text-zinc-100"
                                }
                            >
                                {m.message}
                            </Text>
                        </View>
                    </View>
                ))}
            </ScrollView>

            {/* Input */}
            <View className="flex-row p-2 border-t border-zinc-300 dark:border-zinc-700 items-end gap-2">
                <View className="flex-1">
                    <InputField
                        placeholder="Напишіть повідомлення..."
                        value={msgForm.form.message}
                        onChangeText={msgForm.onChange("message")}
                        onSubmitEditing={send}
                    />
                </View>

                <TouchableOpacity
                    onPress={send}
                    disabled={!isConnected}
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
