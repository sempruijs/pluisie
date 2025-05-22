import { Effect } from "effect";
import { apiUrl } from "./server"; 

export interface LoginRequest {
    email: string;
    password: string;
}

export interface LoginResponse {
    jwt: string;
}

export const Login = (
  payload: LoginRequest
) =>
  Effect.gen(function* () {
    const url = yield* apiUrl("/login");

    const response = yield* Effect.tryPromise({
      try: () =>
        fetch(url, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            accept: "application/json",
          },
          body: JSON.stringify(payload),
        }),
      catch: (err) => new Error("Login request failed: " + String(err)),
    });

    if (!response.ok) {
      const text = yield* Effect.tryPromise({
        try: () => response.text(),
        catch: () => "Unable to read error response",
      });
      throw new Error(`Login failed (${response.status}): ${text}`);
    }

    const data = yield* Effect.tryPromise({
      try: () => response.json() as Promise<LoginResponse>,
      catch: (err) => new Error("Failed to parse JSON: " + String(err)),
    });

    return data;
  });
