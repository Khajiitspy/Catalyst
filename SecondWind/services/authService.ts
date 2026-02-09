import {createApi} from "@reduxjs/toolkit/query/react";
import {IAuthResponse} from "@/types/auth/IAuthResponse";
import {IRegister} from "@/types/auth/IRegister";
import {createBaseQuery} from "@/utils/createBaseQuery";
import {ILogin} from "@/types/auth/ILogin";
import {serialize} from "object-to-formdata";
import type {Dispatch} from "@reduxjs/toolkit";
import {loginSuccess} from "@/store/slices/authSlice";

const handleAuthSuccess = async (
    queryFulfilled: Promise<{ data: {token: string} }>,
    dispatch: Dispatch
) => {
    try {
        const { data } = await queryFulfilled;
        if (data?.token) {
            dispatch(loginSuccess(data.token));
        }
    } catch (error) {
        console.error('Auth error:', error);
    }
};

export const authService = createApi({
    reducerPath: 'api/auth',
    baseQuery: createBaseQuery('Auth'),
    endpoints: (builder) => ({
        login: builder.mutation<IAuthResponse, ILogin>({
            query: (credentials) => ({
                url: 'login',
                method: 'POST',
                body: credentials
            }),
            onQueryStarted: async (_arg, { dispatch, queryFulfilled }) =>
                handleAuthSuccess(queryFulfilled, dispatch)
        }),


        register: builder.mutation<IAuthResponse, IRegister>({
            query: (credentials) => {
                const formData =  serialize(credentials);

                return {
                    url: 'register',
                    method: 'POST',
                    body: formData
                };
            },
            onQueryStarted: async (_arg, { dispatch, queryFulfilled }) =>
                handleAuthSuccess(queryFulfilled, dispatch)
        })

    })
});

export const {
    useLoginMutation,
    useRegisterMutation
} = authService;
