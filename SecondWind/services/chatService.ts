import { createBaseQuery } from "@/utils/createBaseQuery";
import { createApi } from "@reduxjs/toolkit/query/react";
import {IChatType} from "@/types/chat/IChatType";
import {IUserShort} from "@/types/chat/IUserShort";
import { IChatCreate } from "@/types/chat/IChatCreate";
import {IChatListItem} from "@/types/chat/IChatListItem";
import {IMessageItem} from "@/types/chat/IMessageItem";
import {IChatEdit} from "@/types/chat/IChatEdit";
import { IUserSearch } from "@/types/chat/IUserSearch";

export const chatService = createApi({
    reducerPath: "api/chats",
    tagTypes: ["Chats", "Chat"],
    baseQuery: createBaseQuery("chats"),
    endpoints: builder => ({

        getChatTypes: builder.query<IChatType[], void>({
            query: () => "types",
        }),

        getUsers: builder.query<IUserShort[], IUserSearch>({
            query: params => ({
                url: "users",
                params,
            }),
            providesTags: ["Chat"],
        }),

        createChat: builder.mutation<number, IChatCreate>({
            query: body => ({
                url: "",
                method: "POST",
                body,
            }),
            invalidatesTags: ["Chats"],
        }),

        editChat: builder.mutation<void, IChatEdit>({
            query: body => ({
                url: "edit",
                method: "PUT",
                body,
            }),
            invalidatesTags: ["Chats", "Chat"],
        }),

        getMyChats: builder.query<IChatListItem[], void>({
            query: () => "my",
            providesTags: ["Chats"],
        }),

        getChatMessages: builder.query<IMessageItem[], number>({
            query: chatId => `${chatId}/messages`,
            providesTags: ["Chat"],
        }),

        amIAdmin: builder.query<boolean, number>({
            query: chatId => ({
                url: "am-i-admin",
                params: { chatId },
            }),
        }),
    }),
});

export const {
    useGetChatTypesQuery,
    useGetUsersQuery,
    useCreateChatMutation,
    useEditChatMutation,
    useGetMyChatsQuery,
    useGetChatMessagesQuery,
    useAmIAdminQuery,
} = chatService;
