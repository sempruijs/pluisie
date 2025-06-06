// {
//   "date_of_birth": "2024-05-24",
//   "email": "string",
//   "is_super": true,
//   "iva": "string",
//   "name": "string",
//   "password": "string",
//   "phone_number": "string"
// }



import { Effect } from "effect";
import { apiUrl } from "./server"; 

export interface CreateUserRequest {
    email: string;
    password: string;
    date_of_birth: string;
    is_super: boolean;
    iva: string;
    name: string;
    phone_number: string;
}

export const Register = (
  payload: CreateUserRequest
) =>
  Effect.gen(function* () {
    const url = yield* apiUrl("/users");

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
      catch: (err) => new Error("Create User request failed: " + String(err)),
    });

    if (!response.ok) {
      const text = yield* Effect.tryPromise({
        try: () => response.text(),
        catch: () => "Unable to read error response",
      });
      throw new Error(`Registration failed (${response.status}): ${text}`);
    }

    const result = yield* Effect.tryPromise({
        try: async () => {
          if (response.status !== 201) {
            throw new Error(`Unexpected status: ${response.status}`);
          }
          return;
        },
        catch: (err) => new Error("Request failed: " + String(err)),
      });
  });

