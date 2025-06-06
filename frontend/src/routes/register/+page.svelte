<script lang="ts">
    import { Register } from "$lib/ts/register";
    import { Effect } from "effect";
    import { provideServerConfig } from "$lib/ts/server";
    import { serverConfig } from "$lib/config/config.template";

    const user = {
  date_of_birth: "2024-05-24",
  email: "blaaaaaa@gmail.com",
  is_super: true,
  iva: "string",
  name: "bllllll",
  password: "AAAAAAAAA",
  phone_number: "BBBBBB"
} 

    const handleSubmit = () => {
		Effect.runPromise(
			provideServerConfig(serverConfig)(Register(user)),
		)
			.then(() => {
				console.log("Created User");
				window.location.href = "/";
			})
			.catch((error) => {
				alert("Alert, failed to create user. " + error.message);
			});
	};
</script>
<button onclick={() => handleSubmit()}>submit</button>