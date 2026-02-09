import { View, Text, StatusBar, TouchableOpacity, Image } from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { LinearGradient } from "expo-linear-gradient";
import { router } from "expo-router";
import {useAppSelector} from "@/store";
import {IMAGE_URL} from "@/constants/Urls";

export default function HomeScreen() {
    const { user } = useAppSelector(state => state.auth);

    return (
        <View className="flex-1 bg-zinc-50 dark:bg-zinc-950">
            <StatusBar barStyle="default" />

            <LinearGradient
                colors={["rgba(16,185,129,0.35)", "transparent"]}
                className="absolute w-full h-[380px] rounded-full blur-[120px]"
            />

            <SafeAreaView className="flex-1 px-8 justify-between py-16">

                <View className="items-center mt-10">
                    <View className="bg-emerald-500/10 px-4 py-1 rounded-full mb-4 border border-emerald-500/20">
                        <Text className="text-emerald-600 dark:text-emerald-400 text-[10px] font-bold tracking-[3px] uppercase">
                            чат система
                        </Text>
                    </View>

                    <Text className="text-4xl font-black text-zinc-900 dark:text-white tracking-tighter text-center">
                        Привіт,{" "}
                        <Text className="text-emerald-500">
                            {user?.name ?? "Гість"}
                        </Text>
                    </Text>

                    <View className="h-[2px] w-12 bg-emerald-500 my-6 rounded-full" />

                    <Text className="text-zinc-500 dark:text-zinc-400 text-center text-lg leading-7 font-medium px-4">
                        Обери, що хочеш зробити далі
                    </Text>
                </View>

                <View className="items-center my-10">
                    <View className="w-44 h-44 rounded-full bg-emerald-500/10 items-center justify-center overflow-hidden">
                        {user?.image && IMAGE_URL ? (
                            <Image
                                source={{ uri: `${IMAGE_URL}400_${user.image}` }}
                                className="w-full h-full"
                                resizeMode="cover"
                            />
                        ) : (
                            <Text className="text-7xl">👤</Text>
                        )}

                    </View>
                </View>

                <View className="gap-y-5">

                    <View className="relative">
                        <View className="absolute top-1 left-0 right-0 bottom-[-4] bg-emerald-700 rounded-2xl" />

                        <TouchableOpacity
                            activeOpacity={0.85}
                            onPress={() => router.push("/chat/create")}
                            className="bg-emerald-500 py-4 rounded-2xl items-center"
                        >
                            <Text className="text-white text-xl font-bold tracking-tight">
                                Створити новий чат
                            </Text>
                        </TouchableOpacity>
                    </View>

                    <TouchableOpacity
                        activeOpacity={0.85}
                        onPress={() => router.push("/chat/join")}
                        className="border border-zinc-300 dark:border-zinc-700 py-4 rounded-2xl items-center"
                    >
                        <Text className="text-zinc-900 dark:text-zinc-100 text-lg font-semibold">
                            Підключитися до чату
                        </Text>
                    </TouchableOpacity>

                </View>

            </SafeAreaView>
        </View>
    );
}
