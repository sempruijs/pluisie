import { Effect } from "effect";

export interface LoginRequest {
    email: string;
    password: string;
}

export interface LoginResponse {
    jwt: string;
}

export const loginEffect = (
    payload: LoginRequest
): Effect.Effect<LoginResponse, Error> =>
    Effect.tryPromise({
        try: async () => {
            const response = await fetch("http://45.32.236.116:8000/login", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    accept: "application/json",
                },
                body: JSON.stringify(payload),
            });

            if (!response.ok) {
                const text = await response.text();
                throw new Error(`Login failed (${response.status}): ${text}`);
            }

            return await response.json() as LoginResponse;
        },
        catch: (err) => new Error("Login request failed: " + String(err)),
    });