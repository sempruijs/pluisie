<script>
  let selectedSlot = null;

  const timeSlots = [
    { id: 1, time: "11:00–14:00", filled: 1, total: 2 },
    { id: 2, time: "14:00–17:00", filled: 1, total: 2 },
    { id: 3, time: "17:00–21:00", filled: 1, total: 2 }
  ];

  function selectSlot(slotId) {
    selectedSlot = slotId;
  }
</script>

<style>
  .container {
    max-width: 400px;
    margin: auto;
    padding: 1rem;
    font-family: sans-serif;
    background-color: #f9f9f9;
    border-radius: 16px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .header {
    text-align: center;
    font-weight: bold;
    margin-bottom: 1rem;
  }

  .slot {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: #fca311;
    border-radius: 12px;
    padding: 1rem;
    margin: 0.5rem 0;
    cursor: pointer;
    transition: transform 0.2s ease, background-color 0.2s;
  }

  .slot:hover {
    transform: scale(1.02);
    background-color: #ff8800;
  }

  .slot.selected {
    background-color: #ff6700;
  }

  .supervisor {
    margin-top: 1rem;
    padding: 1rem;
    background: #e5e5e5;
    border-radius: 12px;
    font-size: 0.9rem;
  }

  .button {
    display: block;
    width: 100%;
    margin-top: 1rem;
    padding: 0.75rem;
    background-color: #fca311;
    color: white;
    font-weight: bold;
    text-align: center;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .button:hover {
    background-color: #ff8800;
  }
</style>

<div class="container">
  <div class="header">
    <h2>HIDEOUT</h2>
    <div>SCIENCE CAFE</div>
    <h3>VRIJDAG 20 JUNI</h3>
  </div>

  {#each timeSlots as slot}
    <div
      class="slot {selectedSlot === slot.id ? 'selected' : ''}"
      on:click={() => selectSlot(slot.id)}
    >
      <span>{slot.time}</span>
      <span>{slot.filled}/{slot.total}</span>
    </div>
  {/each}

  <div class="supervisor">
    <strong>SUPERVISOR: JAN DE BOER</strong>
    <div>1. BERTJAN</div>
    <div>2. /</div>
  </div>

  <button class="button">AANMELDEN</button>
</div>
