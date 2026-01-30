import React, { useState } from "react";
import { View, TextInput, TouchableOpacity, Text } from "react-native";

interface PasswordFieldProps {
    placeholder: string;
    value: string;
    onChangeText: (text: string) => void;
}

export const PasswordField: React.FC<PasswordFieldProps> = ({ placeholder, value, onChangeText }) => {
    const [showPassword, setShowPassword] = useState(false);

    return (
        <View className="relative">
            <TextInput
                placeholder={placeholder}
                placeholderTextColor="#9ca3af"
                secureTextEntry={!showPassword}
                value={value}
                onChangeText={onChangeText}
                className="w-full bg-zinc-100 dark:bg-zinc-800 rounded-xl px-4 py-4 text-zinc-900 dark:text-white text-base"
            />
            <TouchableOpacity
                onPress={() => setShowPassword(!showPassword)}
                className="absolute right-4 top-1/2 -translate-y-1/2"
            >
                <Text className="text-zinc-500 dark:text-zinc-400">
                    {showPassword ? "Сховати" : "Показати"}
                </Text>
            </TouchableOpacity>
        </View>
    );
};
