import React, { useEffect, useState } from 'react';
import {
    View, Text, FlatList, TextInput, TouchableOpacity,
    Platform, ActivityIndicator, KeyboardAvoidingView
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useLocalSearchParams, useRouter, Stack } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSelector } from 'react-redux';
import { RootState } from "@/store";
import { IChatMessage } from "@/types/сhat/IChatMessage";
import { useGetChatMessagesQuery } from "@/services/chatService";
import { useChatWebSocket } from "@/hooks/useChatWebSocket";

export default function ChatScreen() {
    const router = useRouter();
    const { id, name } = useLocalSearchParams<{ id: string, name: string }>();
    const chatId = Number(id);

    const { user } = useSelector((state: RootState) => state.auth);
    const [inputText, setInputText] = useState('');

    const { messages, setMessages, sendMessage, isConnected } =
        useChatWebSocket(chatId, user?.token);

    const { data: historyMessages, isLoading: isHistoryLoading } = useGetChatMessagesQuery(chatId);

    useEffect(() => {
        if (historyMessages) {
            const sortedMessages = [...historyMessages].reverse();
            setMessages(sortedMessages);
        }
    }, [historyMessages, setMessages]);

    const handleSend = async () => {
        const success = await sendMessage(inputText);
        if (success) {
            setInputText('');
        }
    };

    const renderItem = ({ item }: { item: IChatMessage }) => {
        const isActuallyMine = user?.id === item.userId;
        return (
            <View className={`my-1 mx-3 max-w-[80%] p-3 rounded-2xl ${
                isActuallyMine
                    ? "self-end bg-blue-600 rounded-br-none"
                    : "self-start bg-gray-200 dark:bg-slate-800 rounded-bl-none"
            }`}>
                <Text className={`${isActuallyMine ? "text-white" : "text-black dark:text-white"}`}>
                    {item.message}
                </Text>
                <Text className={`text-[10px] mt-1 text-right ${isActuallyMine ? "text-blue-200" : "text-gray-500"}`}>
                    {new Date(item.createdAt).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                </Text>
            </View>
        );
    };

    return (
        <SafeAreaView className="flex-1 bg-white dark:bg-slate-950" edges={['top']}>
            <Stack.Screen options={{ headerShown: false }} />

            <KeyboardAvoidingView
                behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
                style={{ flex: 1 }}
                keyboardVerticalOffset={Platform.OS === 'ios' ? 0 : 0}
            >
                <View className="flex-row items-center p-4 border-b border-gray-100 dark:border-slate-800 bg-white dark:bg-slate-950">
                    <TouchableOpacity onPress={() => router.back()} className="mr-4 p-1">
                        <Ionicons name="arrow-back" size={28} color="#2563EB" />
                    </TouchableOpacity>
                    <View>
                        <Text className="font-bold text-lg text-black dark:text-white">{name || "Чат"}</Text>
                        <Text className={`text-xs ${isConnected ? "text-green-500" : "text-orange-500"}`}>
                            {isConnected ? "У мережі" : "З'єднання..."}
                        </Text>
                    </View>
                </View>

                <View className="flex-1">
                    {isHistoryLoading && messages.length === 0 ? (
                        <View className="flex-1 justify-center items-center">
                            <ActivityIndicator size="large" color="#2563EB" />
                        </View>
                    ) : (
                        <FlatList
                            data={messages}
                            keyExtractor={(item) => item.id ? item.id.toString() : Math.random().toString()}
                            renderItem={renderItem}
                            inverted={true}
                            contentContainerStyle={{ paddingVertical: 10 }}
                            keyboardDismissMode="interactive"
                            automaticallyAdjustKeyboardInsets={true}
                        />
                    )}
                </View>

                <View className="p-3 border-t border-gray-100 dark:border-slate-800 bg-white dark:bg-slate-950 pb-5">
                    <View className="flex-row items-center">
                        <TextInput
                            className="flex-1 bg-gray-100 dark:bg-slate-900 text-black dark:text-white rounded-2xl px-4 py-3 mr-3 max-h-24"
                            placeholder="Напишіть повідомлення..."
                            placeholderTextColor="#9CA3AF"
                            value={inputText}
                            onChangeText={setInputText}
                            multiline
                        />
                        <TouchableOpacity
                            onPress={handleSend}
                            disabled={!isConnected || !inputText.trim()}
                            className={`w-12 h-12 rounded-full items-center justify-center ${
                                !isConnected || !inputText.trim() ? 'bg-gray-300' : 'bg-blue-600'
                            }`}
                        >
                            <Ionicons name="send" size={20} color="white" style={{ paddingLeft: 2 }} />
                        </TouchableOpacity>
                    </View>
                </View>
            </KeyboardAvoidingView>
        </SafeAreaView>
    );
}
