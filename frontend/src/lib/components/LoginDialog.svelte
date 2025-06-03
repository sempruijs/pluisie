<script lang="ts">
	import Button from "./Button.svelte";
	import { Effect } from "effect";
	import { Login } from "$lib/ts/login";
	import { provideServerConfig, ServerConfig } from "$lib/ts/server";
	import { serverConfig } from "$lib/config/config.template";

	const form = $state({
		email: "",
		password: "",
	});

	const handleLogin = (email: string, password: string) => {
		Effect.runPromise(
			provideServerConfig(serverConfig)(Login({ email, password })),
		)
			.then(({ jwt }) => {
				console.log("JWT:", jwt);
				localStorage.setItem("jwt", jwt);
				window.location.href = "/selectcafe";
			})
			.catch((error) => {
				alert("Login failed: " + error.message);
			});
	};
</script>

<div
	class="bg-white h-90 w-full max-w-lg md:max-w-xl mx-3 p-8 rounded-lg shadow-2xl"
>
	<div class="flex2 my-4">
		<h1 class="pl-1 font-semibold">Persoonlijke e-mail:</h1>
		<input
			type="text"
			bind:value={form.email}
			class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 shadow-gray-200 rounded-lg outline-none selecttext"
			placeholder="Persoonlijke e-mail"
		/>
	</div>
	<div class="flex2 my-4">
		<h1 class="pl-1 font-semibold">Wachtwoord:</h1>
		<input
			type="password"
			bind:value={form.password}
			class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 shadow-gray-200 rounded-lg outline-none selecttext"
			placeholder="Wachtwoord"
		/>
	</div>
	<div class="flex justify-between my-4">
		<Button on:click={() => handleLogin(form.email, form.password)}
			>Login</Button
		>
		<Button color="red">Reset Account</Button>
	</div>
	<div class="flex w-full h-0.5 bg-gray-300 my-5">
		<div class="w-full flex flex-col items-center">
			<h3 class="my-2 font-semibold text-center">Of:</h3>
			<div class="flex justify-center">
				<a href="/register">
					<Button color="orange" padding="lg">
						Registreer je nu als vrijwilliger
					</Button>
				</a>
			</div>
		</div>
	</div>
</div>
