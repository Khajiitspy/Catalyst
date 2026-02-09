export interface IChatItem {
    id: number;
    name: string | null;
    chatTypeName: string | null;
    image?: string | null;
    lastMessage?: string | null;
    unread?: number;
    time?: string;
}
