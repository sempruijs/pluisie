<script lang="ts">
    import { createEventDispatcher } from "svelte";
  
    export let color: string = "green";
    export let padding: string = "default";
    export let width: string = "auto";
    export let className: string = "";
  
    const dispatch = createEventDispatcher();
  
    let color_style = "btn-green";
    let padding_style = "p-3 px-7.5";
    let width_style = "w-auto";
  
    if (color === "orange") color_style = "btn-orange";
    if (color === "red") color_style = "btn-red";
    if (color === "yellow") color_style = "btn-yellow";
  
    if (padding === "lg") padding_style = "px-10 py-5";
    if (width === "max") width_style = "w-full";
  </script>
  
  <button
    on:click={(e) => dispatch("click", e)}
    class="{color_style} {width_style} max-h-6 {padding_style} border shadow-xl shadow-gray-300 rounded-lg flex items-center justify-center text-center hoverbutton {className}"
  >
    <slot />
  </button>
  