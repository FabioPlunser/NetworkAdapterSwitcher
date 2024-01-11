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

<div>
  {#each adapters.sort((a, b) => a.Name.localeCompare(b.Name)) as adapter}
    <div>
      <h2>{adapter.Name}</h2>
      <input
        type="checkbox"
        class="toggle"
        on:change={async (e) => handleAdapterStatusChange(e, adapter.Name)}
        checked={getAdapterStatus(adapter)}
      />
    </div>
  {/each}
</div>
