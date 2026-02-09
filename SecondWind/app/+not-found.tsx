import React from "react";
import {
    View,
    Text,
    StatusBar,
    ScrollView,
    KeyboardAvoidingView,
    Platform,
    TouchableOpacity,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { router } from "expo-router";

export default function NotFoundScreen() {
    return (
        <View className="flex-1 bg-zinc-50 dark:bg-zinc-950">
            <StatusBar barStyle="default" />

            <SafeAreaView className="flex-1 px-6 py-12">
                <KeyboardAvoidingView
                    style={{ flex: 1 }}
                    behavior={Platform.OS === "ios" ? "padding" : "height"}
                >
                    <ScrollView
                        contentContainerStyle={{
                            flexGrow: 1,
                            justifyContent: "center",
                        }}
                        keyboardShouldPersistTaps="handled"
                    >
                        <View className="items-center mb-12">
                            <Text className="text-5xl font-black text-zinc-900 dark:text-white tracking-tighter text-center">
                                Щось пішло не так
                            </Text>

                            <View className="h-[2px] w-12 bg-emerald-500 my-6 rounded-full" />

                            <Text className="text-zinc-500 dark:text-zinc-400 text-center text-lg leading-7 font-medium px-4">
                                Сторінку не знайдено або маршрут недоступний
                            </Text>
                        </View>

                        {/* КНОПКА НА ГОЛОВНУ */}
                        <TouchableOpacity
                            activeOpacity={0.85}
                            onPress={() => router.replace("/")}
                            className="bg-emerald-500 py-4 rounded-2xl items-center mx-6"
                        >
                            <Text className="text-white text-lg font-bold">
                                На головну
                            </Text>
                        </TouchableOpacity>
                    </ScrollView>
                </KeyboardAvoidingView>
            </SafeAreaView>
        </View>
    );
}
