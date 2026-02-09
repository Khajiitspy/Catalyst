import { FC, useState } from "react";
import { View, Text, ScrollView, TouchableOpacity, Modal } from "react-native";

import { InputField } from "@/components/form/InputField";
import { useForm } from "@/hooks/useForm";

import {
    useEditChatMutation,
    useGetUsersQuery,
} from "@/services/chatService";
import { IChatEdit } from "@/types/chat/IChatEdit";

interface Props {
    chatId: number;
    visible: boolean;
    onClose: () => void;
}

const EditChatModal: FC<Props> = ({ chatId, visible, onClose }) => {
    const [editChat, { isLoading }] = useEditChatMutation();

    const [addIds, setAddIds] = useState<number[]>([]);
    const [removeIds, setRemoveIds] = useState<number[]>([]);

    const editForm = useForm<IChatEdit>({
        id: chatId,
        name: "",
        addUserIds: [],
        removeUserIds: [],
    });

    const { data: members } = useGetUsersQuery(
        { chatId },
        { skip: !visible }
    );

    const { data: searchUsers } = useGetUsersQuery(
        { query: editForm.form.name },
        { skip: !visible || (editForm.form.name?.length ?? 0) < 2 }
    );

    const toggle = (
        id: number,
        list: number[],
        set: (v: number[]) => void
    ) => {
        set(list.includes(id) ? list.filter(x => x !== id) : [...list, id]);
    };

    const save = async () => {
        await editChat({
            id: chatId,
            name: editForm.form.name?.trim() || undefined,
            addUserIds: addIds,
            removeUserIds: removeIds,
        });

        setAddIds([]);
        setRemoveIds([]);
        onClose();
    };

    return (
        <Modal visible={visible} transparent animationType="fade">
            <View className="flex-1 bg-black/50 items-center justify-center">
                <ScrollView className="w-[90%] bg-white dark:bg-zinc-900 rounded-xl p-4">

                    <Text className="text-lg font-semibold mb-3 text-zinc-900 dark:text-zinc-100">
                        Редагувати чат
                    </Text>

                    <InputField
                        placeholder="Назва чату"
                        value={editForm.form.name ?? ""}
                        onChangeText={editForm.onChange("name")}
                    />

                    <Text className="mt-4 mb-1 font-semibold text-zinc-700 dark:text-zinc-300">
                        Поточні учасники:
                    </Text>

                    {members?.map(u => (
                        <View
                            key={u.id}
                            className="flex-row justify-between items-center bg-zinc-200 dark:bg-zinc-800 p-2 rounded-lg mb-1"
                        >
                            <Text className="text-zinc-900 dark:text-zinc-100">
                                {u.name}
                            </Text>

                            <TouchableOpacity
                                onPress={() =>
                                    toggle(u.id, removeIds, setRemoveIds)
                                }
                            >
                                <Text
                                    className={`font-semibold ${
                                        removeIds.includes(u.id)
                                            ? "text-red-500"
                                            : "text-emerald-500"
                                    }`}
                                >
                                    {removeIds.includes(u.id)
                                        ? "Видалено"
                                        : "Видалити"}
                                </Text>
                            </TouchableOpacity>
                        </View>
                    ))}

                    {searchUsers?.map(u => (
                        <TouchableOpacity
                            key={u.id}
                            onPress={() =>
                                toggle(u.id, addIds, setAddIds)
                            }
                            className="flex-row justify-between items-center bg-zinc-200 dark:bg-zinc-800 p-2 rounded-lg mb-1"
                        >
                            <Text className="text-zinc-900 dark:text-zinc-100">
                                {u.name}
                            </Text>

                            <Text
                                className={`font-semibold ${
                                    addIds.includes(u.id)
                                        ? "text-emerald-500"
                                        : "text-zinc-500"
                                }`}
                            >
                                {addIds.includes(u.id)
                                    ? "Додано"
                                    : "Додати"}
                            </Text>
                        </TouchableOpacity>
                    ))}

                    <View className="flex-row justify-end gap-3 mt-3">
                        <TouchableOpacity onPress={onClose}>
                            <Text className="text-zinc-500">Скасувати</Text>
                        </TouchableOpacity>

                        <TouchableOpacity onPress={save} disabled={isLoading}>
                            <Text className="text-emerald-500 font-semibold">
                                Зберегти
                            </Text>
                        </TouchableOpacity>
                    </View>

                </ScrollView>
            </View>
        </Modal>
    );
};

export default EditChatModal;
