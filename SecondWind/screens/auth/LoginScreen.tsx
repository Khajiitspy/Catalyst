import React from "react";
import {
    View,
    Text,
    StatusBar,
    TouchableOpacity,
    ScrollView,
    KeyboardAvoidingView,
    Platform,
} from "react-native";
import { SafeAreaView } from "react-native-safe-area-context";
import { InputField } from "@/components/form/InputField";
import { PasswordField } from "@/components/form/PasswordField";
import { router } from "expo-router";
import { useLoginMutation } from "@/services/authService";
import { ILogin } from "@/types/auth/ILogin";
import { useForm } from "@/hooks/useForm";

export default function LoginScreen() {
    const { form, onChange } = useForm<ILogin>({
        email: "",
        password: "",
    });

    const [login, { isLoading }] = useLoginMutation();

    const onSubmit = async () => {
        try {
            const res = await login(form).unwrap();
            router.replace("/chat/home")
        } catch (e) {
            console.log("Login error:", e);
        }
    };

    return (
        <View className="flex-1 bg-zinc-50 dark:bg-zinc-950">
            <StatusBar barStyle="default" />

            <SafeAreaView className="flex-1 p-6">
                <KeyboardAvoidingView
                    style={{ flex: 1 }}
                    behavior={Platform.OS === "ios" ? "padding" : "height"}
                >
                    {/*<ScrollView*/}
                    {/*    contentContainerStyle={{*/}
                    {/*        flexGrow: 1,*/}
                    {/*        justifyContent: "center",*/}
                    {/*    }}*/}
                    {/*    keyboardShouldPersistTaps="handled"*/}
                    {/*>*/}
                        <View className="items-center mb-12">
                            <Text className="text-5xl font-black text-zinc-900 dark:text-white tracking-tighter">
                                Вхід
                            </Text>
                            <View className="h-[2px] w-12 bg-emerald-500 my-6 rounded-full" />
                            <Text className="text-zinc-500 dark:text-zinc-400 text-center text-lg leading-7 font-medium px-4">
                                Увійдіть у свій акаунт, щоб продовжити
                            </Text>
                        </View>

                        <View className="gap-y-4">
                            <InputField
                                placeholder="Пошта"
                                keyboardType="email-address"
                                textContentType="emailAddress"
                                autoCapitalize="none"
                                value={form.email}
                                onChangeText={onChange("email")}
                            />

                            <PasswordField
                                placeholder="Пароль"
                                value={form.password}
                                onChangeText={onChange("password")}
                            />

                            <TouchableOpacity className="self-end mt-1">
                                <Text
                                    onPress={() => router.push("/test")}
                                    className="text-emerald-500 dark:text-emerald-400 font-medium"
                                >
                                    Забув пароль?
                                </Text>
                            </TouchableOpacity>
                        </View>

                        <View className="mt-8">
                            <TouchableOpacity
                                className="bg-emerald-500 py-5 rounded-2xl items-center"
                                onPress={onSubmit}
                                disabled={isLoading}
                            >
                                <Text className="text-white text-xl font-bold">
                                    {isLoading ? "Завантаження..." : "Увійти"}
                                </Text>
                            </TouchableOpacity>
                        </View>
                    {/*</ScrollView>*/}
                </KeyboardAvoidingView>
            </SafeAreaView>
        </View>
    );
}
