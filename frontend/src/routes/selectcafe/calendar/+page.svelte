<script>
  import Calendar from "$lib/components/Calendar.svelte";
  import Header from "$lib/components/Header.svelte";
  import Rooster from "$lib/components/Rooster.svelte";

  let selectedDate = new Date().toISOString().slice(0, 10);

  function updateSelectedDate(event) {
    selectedDate = event.detail;
  }
</script>
<main class="flex-1 flex items-center justify-center bg-gradient-plant p-4
             flex-col md:flex-row md:gap-6">
  <Header />

  <Calendar on:selectedDate={updateSelectedDate} class="w-full md:w-auto" />
  <Rooster {selectedDate} class="w-full md:w-auto" />
</main>
