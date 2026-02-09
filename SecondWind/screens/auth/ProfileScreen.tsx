import React, { useState } from 'react';
import { View, Text, Image, TouchableOpacity, ScrollView } from 'react-native';
import { useDispatch, useSelector } from 'react-redux';
import { useRouter } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { RootState } from '@/store';
import { logout } from '@/store/authSlice';
import { ActionModal } from '@/components/modals/ActionModal';
import {APP_URLS} from "@/constants/Urls";

export default function ProfileScreen() {
    const router = useRouter();
    const dispatch = useDispatch();
    const insets = useSafeAreaInsets();

    const [modalVisible, setModalVisible] = useState(false);
    const user = useSelector((state: RootState) => state.auth.user);

    const handleLogoutConfirm = () => {
        dispatch(logout());
        router.replace('/');
    };

    if (!user) return null;

    return (
        <>
            <ScrollView
                className="flex-1 bg-gray-100 dark:bg-slate-950"
                contentContainerStyle={{ paddingTop: insets.top }}
            >
                <View className="bg-white dark:bg-slate-900 p-6 items-center border-b border-gray-200 dark:border-slate-800 mb-4">
                    <Image
                        source={{ uri: `${APP_URLS.IMAGES_400_URL}${user.image}` }}
                        className="w-24 h-24 rounded-full mb-3 bg-gray-200 border-2 border-white dark:border-slate-700"
                    />
                    <Text className="text-2xl font-bold text-black dark:text-white text-center">
                        {user.name || "Користувач"}
                    </Text>
                    <Text className="text-gray-500 dark:text-gray-400 text-sm">
                        {user.email}
                    </Text>
                    <Text className="text-blue-500 text-sm mt-1 font-medium">Online</Text>
                </View>

                <View className="bg-white dark:bg-slate-900 px-4 mb-4">
                    <View className="py-4 border-b border-gray-100 dark:border-slate-800">
                        <Text className="text-blue-500 text-xs uppercase mb-1 font-bold">Email</Text>
                        <Text className="text-black dark:text-white text-base">{user.email}</Text>
                    </View>
                </View>

                <View className="bg-white dark:bg-slate-900 px-4 mb-10">
                    <TouchableOpacity
                        onPress={() => setModalVisible(true)}
                        className="flex-row items-center py-4 active:opacity-70"
                    >
                        <Ionicons name="log-out-outline" size={24} color="#ef4444" />
                        <Text className="text-red-500 text-lg ml-3 font-bold">Вийти з акаунту</Text>
                    </TouchableOpacity>
                </View>
            </ScrollView>

            <ActionModal
                visible={modalVisible}
                onClose={() => setModalVisible(false)}
                onConfirm={handleLogoutConfirm}
                title="Вихід"
                description="Ви впевнені, що хочете вийти з облікового запису?"
                confirmText="Так, вийти"
                variant="danger"
            />
        </>
    );
}
