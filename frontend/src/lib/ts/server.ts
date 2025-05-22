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