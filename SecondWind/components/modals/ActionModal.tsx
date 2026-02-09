import React, { useEffect, useRef, useState } from "react";
import {
    View, Text, Pressable, Modal,
    TouchableWithoutFeedback, Animated, PanResponder, Dimensions, TouchableOpacity
} from "react-native";

const SCREEN_HEIGHT = Dimensions.get("window").height;

type ActionModalProps = {
    visible: boolean;
    onClose: () => void;
    onConfirm: () => void;
    title: string;
    description?: string;
    confirmText?: string;
    cancelText?: string;
    variant?: 'danger' | 'primary';
};

export const ActionModal = ({
                                visible,
                                onClose,
                                onConfirm,
                                title,
                                description,
                                confirmText = "Так",
                                cancelText = "Скасувати",
                                variant = 'primary'
                            }: ActionModalProps) => {

    const [showModal, setShowModal] = useState(visible);
    const translateY = useRef(new Animated.Value(SCREEN_HEIGHT)).current;

    useEffect(() => {
        if (visible) {
            setShowModal(true);
            Animated.spring(translateY, {
                toValue: 0,
                useNativeDriver: true,
                tension: 50,
                friction: 9,
            }).start();
        } else {
            handleClose();
        }
    }, [visible]);

    const handleClose = () => {
        Animated.timing(translateY, {
            toValue: SCREEN_HEIGHT,
            duration: 250,
            useNativeDriver: true,
        }).start(() => {
            setShowModal(false);
            onClose();
        });
    };

    const panResponder = useRef(
        PanResponder.create({
            onStartShouldSetPanResponder: () => true,
            onMoveShouldSetPanResponder: (_, gestureState) => gestureState.dy > 5,
            onPanResponderMove: (_, gestureState) => {
                if (gestureState.dy > 0) {
                    translateY.setValue(gestureState.dy);
                }
            },
            onPanResponderRelease: (_, gestureState) => {
                if (gestureState.dy > 100 || gestureState.vy > 0.5) {
                    handleClose();
                } else {
                    Animated.spring(translateY, {
                        toValue: 0,
                        useNativeDriver: true,
                        bounciness: 5,
                    }).start();
                }
            },
        })
    ).current;

    const buttonColorClass = variant === 'danger' ? 'bg-red-500' : 'bg-blue-600';

    return (
        <Modal
            animationType="fade"
            transparent={true}
            visible={showModal}
            onRequestClose={handleClose}
            statusBarTranslucent={true}
        >
            <View className="flex-1 bg-black/60 justify-end">
                <TouchableWithoutFeedback onPress={handleClose}>
                    <View className="absolute inset-0" />
                </TouchableWithoutFeedback>

                <Animated.View
                    style={{ transform: [{ translateY }] }}
                    className="bg-white dark:bg-slate-900 rounded-t-[40px] pb-12 shadow-2xl overflow-hidden"
                >
                    <View {...panResponder.panHandlers} className="w-full items-center pt-4 pb-2 bg-transparent">
                        <View className="w-12 h-1.5 bg-gray-200 dark:bg-slate-700 rounded-full" />
                    </View>

                    <View className="px-6 items-center">
                        <Text className="text-2xl font-bold text-gray-900 dark:text-white mt-2 mb-2 text-center">
                            {title}
                        </Text>

                        {description && (
                            <Text className="text-gray-500 dark:text-gray-400 text-center mb-8 text-base leading-6 px-4">
                                {description}
                            </Text>
                        )}

                        <View className="w-full gap-y-3">
                            <TouchableOpacity
                                onPress={() => {
                                    onConfirm();
                                    handleClose();
                                }}
                                className={`${buttonColorClass} py-4 rounded-2xl items-center shadow-sm active:opacity-90`}
                            >
                                <Text className="text-white font-bold text-lg">{confirmText}</Text>
                            </TouchableOpacity>

                            <TouchableOpacity
                                onPress={handleClose}
                                className="bg-gray-100 dark:bg-slate-800 py-4 rounded-2xl items-center active:bg-gray-200 dark:active:bg-slate-700"
                            >
                                <Text className="text-gray-700 dark:text-gray-200 font-bold text-lg">{cancelText}</Text>
                            </TouchableOpacity>
                        </View>
                    </View>
                </Animated.View>
            </View>
        </Modal>
    );
};
