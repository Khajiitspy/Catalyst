export interface IUserIdentity {
    id: number;
    name: string;
    email: string;
    image?: string;
    token: string;
    role: string[];
}
