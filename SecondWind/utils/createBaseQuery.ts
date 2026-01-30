import {
  fetchBaseQuery,
  BaseQueryFn,
  FetchArgs,
  FetchBaseQueryError
} from "@reduxjs/toolkit/query/react";
import { BASE_URL } from "@/constants/Urls";

export const createBaseQuery = (
  endpoint: string
): BaseQueryFn<string | FetchArgs, unknown, FetchBaseQueryError> => {

  const rawBaseQuery = fetchBaseQuery({
    baseUrl: `${BASE_URL}/api/${endpoint}/`,
  });

  return async (args, api, extraOptions) => {
    console.log("➡️ RTK REQUEST:", args);

    const result = await rawBaseQuery(args, api, extraOptions);

    console.log("⬅️ RTK RESPONSE:", result);

    return result;
  };
};
