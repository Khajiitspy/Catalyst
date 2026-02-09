import React from 'react';
import { View, Text, Modal, TouchableOpacity, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';

type ModalType = 'success' | 'error' | 'info';

interface StatusModalProps {
    visible: boolean;
    onClose: () => void;
    type?: ModalType;
    title: string;
    description?: string;
    buttonText?: string;
}

export const StatusModal = ({
                                visible,
                                onClose,
                                type = 'info',
                                title,
                                description,
                                buttonText = "Зрозуміло"
                            }: StatusModalProps) => {

    const config = {
        success: {
            icon: 'checkmark-circle' as keyof typeof Ionicons.glyphMap,
            color: 'text-green-500',
            bgColor: 'bg-green-100 dark:bg-green-900/30',
            btnColor: 'bg-green-500 active:bg-green-600',
        },
        error: {
            icon: 'alert-circle' as keyof typeof Ionicons.glyphMap,
            color: 'text-red-500',
            bgColor: 'bg-red-100 dark:bg-red-900/30',
            btnColor: 'bg-red-500 active:bg-red-600',
        },
        info: {
            icon: 'information-circle' as keyof typeof Ionicons.glyphMap,
            color: 'text-blue-500',
            bgColor: 'bg-blue-100 dark:bg-blue-900/30',
            btnColor: 'bg-blue-500 active:bg-blue-600',
        }
    };

    const currentConfig = config[type];

    return (
        <Modal
            animationType="fade"
            transparent={true}
            visible={visible}
            onRequestClose={onClose}
            statusBarTranslucent={true}
        >
            <View className="flex-1 justify-center items-center bg-black/60 px-6">
                <Pressable
                    className="absolute inset-0 w-full h-full"
                    onPress={onClose}
                />

                <View className="bg-white dark:bg-slate-900 w-full max-w-sm rounded-3xl p-6 items-center shadow-2xl">
                    <View className={`w-16 h-16 rounded-full items-center justify-center mb-4 ${currentConfig.bgColor}`}>
                        <Ionicons
                            name={currentConfig.icon}
                            size={32}
                            color={type === 'error' ? '#ef4444' : type === 'success' ? '#22c55e' : '#3b82f6'}
                        />
                    </View>

                    <Text className="text-xl font-bold text-black dark:text-white text-center mb-2">
                        {title}
                    </Text>

                    {description && (
                        <Text className="text-gray-500 dark:text-gray-400 text-center mb-6 leading-5">
                            {description}
                        </Text>
                    )}

                    <TouchableOpacity
                        onPress={onClose}
                        className={`w-full py-3.5 rounded-xl items-center ${currentConfig.btnColor}`}
                    >
                        <Text className="text-white font-bold text-lg">{buttonText}</Text>
                    </TouchableOpacity>
                </View>
            </View>
        </Modal>
    );
};
