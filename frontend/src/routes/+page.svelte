<script>
import Button from './../lib/components/Button.svelte';
</script>

<main class="flex1 center-v center-h flex-1 bg-gradient">
		<div class="bg-white h-75 w-full max-w-lg md:max-w-xl mx-3 p-8 rounded-lg shadow-2xl">
			<div class="flex2 my-4">
				<h1 class="pl-1 font-semibold">E-mail:</h1>
				<input type="text"
				class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 shadow-gray-200 rounded-lg outline-none selecttext" 
				/>
			</div>
			<div class="flex justify-between my-4">
				<Button>
					Login
				</Button>

				<Button color='red'>
					Reset Account
				</Button>
			</div>
			<div class="flex w-full h-0.5 bg-gray-300 my-5"  />
			<div>
				<h1 class="font-semibold text-center">Of:</h1>
			
				<div class="flex justify-center">
					<a href="/register">
					<Button color="orange" padding="lg">
						Registreer je nu als vrijwilliger
					</Button>
					</a>
				</div>
			</div>
		</div>	
</main>