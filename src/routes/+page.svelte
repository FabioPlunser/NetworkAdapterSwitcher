<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let adapters: any[] = [];

  async function getAdapters() {
    const response = await invoke("get_all_network_adapters");
    adapters = JSON.parse(response);
  }
  onMount(async () => {
    getAdapters();
  });

  $: console.log(adapters);

  // function toggleAdapterStatus(adapter: any) {
  //   invoke("toggle_adapter_status", { adapter });
  // }

  function getAdapterStatus(adapter: any) {
    return adapter.Status === "Up";
  }

  async function handleAdapterStatusChange(event: any, adapterName: string) {
    let response = "";
    if (event.target.checked) {
      console.log("enable_adapter", { adapterName });
      response = await invoke("enable_adapter", { adapterName });
    } else {
      console.log("disable_adapter", { adapterName });
      response = await invoke("disable_adapter", { adapterName });
    }
    console.log(response);

    await getAdapters();
  }
</script>

<div class="text-2xl font-bold flex justify-center">
  <h1>NetworkAdapterSwitcher</h1>
</div>

{#if adapters.length === 0}
  <div class="flex justify-center">
    <span class="loading text-primary loading-dots loading-lg text-5xl"></span>
  </div>
{/if}
<div class="flex justify-center mx-auto text-2xl text-white">
  <div class="flex flex-col justify-center mx-auto">
    {#each adapters.sort((a, b) => a.Name.localeCompare(b.Name)) as adapter}
      <div>
        <label class="label cursor-pointer">
          <span class=" mr-2">{adapter.Name}</span>
          <input
            type="checkbox"
            class="toggle toggle-primary"
            on:change={async (e) => handleAdapterStatusChange(e, adapter.Name)}
            checked={getAdapterStatus(adapter)}
          />
        </label>
      </div>
    {/each}
  </div>
</div>
