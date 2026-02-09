import { View, TextInput } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { ReactNode } from "react";
import { useColorScheme } from "nativewind";

type Props = {
    icon: keyof typeof Ionicons.glyphMap;
    value: string;
    onChangeText: (text: string) => void;
    placeholder: string;
    secureTextEntry?: boolean;
    keyboardType?: "default" | "email-address";
    rightIcon?: ReactNode;
};

export default function IconInput({
                                      icon,
                                      value,
                                      onChangeText,
                                      placeholder,
                                      secureTextEntry,
                                      keyboardType,
                                      rightIcon,
                                  }: Props) {
    const { colorScheme } = useColorScheme();
    const isDark = colorScheme === "dark";

    return (
        <View
            className="flex-row items-center border border-gray-200 dark:border-slate-800 bg-gray-50 dark:bg-slate-900 rounded-2xl px-4 py-4 mb-4"
        >
            <Ionicons
                name={icon}
                size={22}
                color={isDark ? "#94A3B8" : "#6B7280"}
            />

            <TextInput
                className="flex-1 ml-3 text-black dark:text-white text-base"
                placeholder={placeholder}
                placeholderTextColor={isDark ? "#64748B" : "#94A3B8"}
                value={value}
                onChangeText={onChangeText}
                secureTextEntry={secureTextEntry}
                keyboardType={keyboardType}
                autoCapitalize="none"
            />

            {rightIcon}
        </View>
    );
}
