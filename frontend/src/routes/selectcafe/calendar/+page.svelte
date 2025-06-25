<script>
  import Calendar from "$lib/components/Calendar.svelte";
  import Header from "$lib/components/Header.svelte";
  import Rooster from "$lib/components/Rooster.svelte";

  let selectedDate = new Date().toISOString().slice(0, 10);

  function updateSelectedDate(event) {
    selectedDate = event.detail;
  }
</script>
<main class="flex-1 flex items-center justify-center bg-gradient-plant">
  <Header />
  <Calendar on:selectedDate={updateSelectedDate} />
  <Rooster {selectedDate} />
</main>