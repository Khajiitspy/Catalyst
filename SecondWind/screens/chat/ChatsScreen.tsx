import React from 'react';
import { View, Text, FlatList, Image, TouchableOpacity, RefreshControl, ActivityIndicator } from 'react-native';
import { SafeAreaView } from "react-native-safe-area-context";
import { useRouter } from 'expo-router';
import { APP_URLS } from "@/constants/Urls";
import { useGetMyChatsQuery } from "@/services/chatService";
import { IChatItem } from "@/types/сhat/IChatItem";

export default function ChatsScreen() {
    const router = useRouter();
    const { data: chats = [], isLoading, refetch } = useGetMyChatsQuery();

    const renderChatItem = ({ item }: { item: IChatItem }) => {
        const chatName = item.name || "Без назви";
        const lastMsg = item.lastMessage || "Немає повідомлень";
        const timeDisplay = item.time || "";
        const unreadCount = item.unread || 0;

        return (
            <TouchableOpacity
                className="flex-row items-center px-4 py-3 active:bg-gray-50 dark:active:bg-slate-900"
                onPress={() => {
                    router.push({
                        pathname: "/chat/[id]",
                        params: { id: item.id, name: chatName }
                    });
                }}
            >
                <Image
                    source={
                        item.image
                            ? { uri: `${APP_URLS.IMAGES_200_URL}${item.image}` }
                            : require('@/assets/images/user.png')
                    }
                    className="w-14 h-14 rounded-full bg-gray-200 dark:bg-slate-800"
                />

                <View className="flex-1 ml-3 border-b border-gray-100 dark:border-slate-800 pb-3 justify-center">
                    <View className="flex-row justify-between mb-1">
                        <Text className="font-bold text-lg text-black dark:text-white">
                            {chatName}
                        </Text>
                        <Text className="text-gray-400 text-xs">
                            {timeDisplay}
                        </Text>
                    </View>

                    <View className="flex-row justify-between items-center">
                        <Text className="text-gray-500 dark:text-gray-400 text-sm flex-1 mr-2" numberOfLines={1}>
                            {lastMsg}
                        </Text>

                        {unreadCount > 0 && (
                            <View className="bg-blue-600 rounded-full min-w-[20px] h-5 items-center justify-center px-1.5">
                                <Text className="text-white text-[10px] font-bold">
                                    {unreadCount}
                                </Text>
                            </View>
                        )}
                    </View>
                </View>
            </TouchableOpacity>
        );
    };

    return (
        <SafeAreaView className="flex-1 bg-white dark:bg-slate-950" edges={['top']}>
            <View className="flex-1">
                <Text className="text-3xl font-bold px-4 py-4 text-black dark:text-white">
                    Чати
                </Text>

                {isLoading ? (
                    <View className="flex-1 justify-center items-center">
                        <ActivityIndicator size="large" color="#2563EB" />
                    </View>
                ) : (
                    <FlatList
                        data={chats}
                        keyExtractor={(item) => item.id.toString()}
                        contentContainerClassName="pb-20"
                        renderItem={renderChatItem}
                        ListEmptyComponent={
                            <View className="mt-10 items-center">
                                <Text className="text-gray-500">У вас поки немає чатів</Text>
                            </View>
                        }
                        refreshControl={
                            <RefreshControl refreshing={isLoading} onRefresh={refetch} />
                        }
                    />
                )}
            </View>
        </SafeAreaView>
    );
}
