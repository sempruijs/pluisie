import { Effect, Context } from "effect";

export interface ServerConfigSchema {
    readonly ip: string;
    readonly port: number;
}

export class ServerConfig extends Context.Tag("ServerConfig")<
    ServerConfig,
    ServerConfigSchema
>() { }

export function provideServerConfig(config: ServerConfigSchema) {
    return Effect.provideService(ServerConfig, config);
}

export const apiUrl = (path: string): Effect.Effect<string, never, ServerConfig> =>
    Effect.gen(function* () {
        const { ip, port } = yield* ServerConfig;

        const baseUrl = `http://${ip}:${port}`;
        const cleanedPath = path.startsWith("/") ? path : `/${path}`;
        return `${baseUrl}${cleanedPath}`;
    })