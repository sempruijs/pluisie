import { Effect } from "effect";
import { apiUrl } from "./server";
import type { User } from "./domain";

export const GetUser = (jwt: string) =>
  Effect.gen(function* () {
    const url = yield* apiUrl("/users");

    const response = yield* Effect.tryPromise({
      try: () =>
        fetch(url, {
          method: "GET",
          headers: {
            accept: "application/json",
            Authorization: `Bearer ${jwt}`,
          },
        }),
      catch: (err) => new Error("User request failed: " + String(err)),
    });

    if (!response.ok) {
      const text = yield* Effect.tryPromise({
        try: () => response.text(),
        catch: () => "Unable to read error response",
      });
      throw new Error(`Fetching user failed (${response.status}): ${text}`);
    }

    const user = yield* Effect.tryPromise({
      try: () => response.json() as Promise<User>,
      catch: (err) => new Error("Failed to parse JSON: " + String(err)),
    });

    return user;
  });
