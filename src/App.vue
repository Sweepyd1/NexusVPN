<script setup>
import ProtectIcon from "./svg/protect.vue";
import NoProtectionIcon from "./svg/noprotect.vue";

import newConfig from "./components/newConfig.vue";
import { ref } from "vue";

const isOpenModal = ref(false);

function openModal(){
  isOpenModal.value = true;
}
function closeModal() {
    isOpenModal.value = false;
}
</script>

<template>
  <newConfig v-if="isOpenModal" @close="closeModal"></newConfig>
  <div class="container">
    <div class="logo">
      <ProtectIcon></ProtectIcon>
      <span>Cosmic VPN Shield</span>
    </div>

    <div class="main">
      <div class="main_block">
        <div class="isconnected">
          
          <span>Not Protected</span>
          <NoProtectionIcon></NoProtectionIcon>
          <span class="small">Not connected</span>
        </div>

        <div class="config_list">
          <span class="small">No VPN configurations yet</span>
          <span class="small">Add a new configuration to get <br>started</span>
        </div>
      </div>

      <div class="add_config" @click="openModal">
        <span>Add New Configuration</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
span{
  font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
}
body {
  margin: 0;
  padding: 0;
}

.container {
  padding-top: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  height: 100vh;
  background-color: #2c2c44;

  .logo {
    display: flex;
    gap: 10px;
    align-items: center;
    span {
      color: white;
      font-size: 1.5rem;
      line-height: 2rem;
      font-family: "UI Sans Serif", sans-serif;
      font-weight: 600;
    }
  }

  .main {
    margin-top: 30px;
    width: 50%;
    display: flex;
    flex-direction: column;
    gap: 25px;

    .main_block {
      display: flex;
      gap: 25px;
    }

    .isconnected {
      width: 45%;
      height: 25vh;
      background-color: #353550;
      display: flex;
      flex-direction: column;
      gap: 20px;
      align-items: center;
      justify-content: center;
      border: solid 1px rgba(114, 129, 225, 0.2);
      border-radius: 10px;

      span {
        color: white;
        font-size: 1.5rem;
        line-height: 2rem;
        font-family: "UI Sans Serif", sans-serif;
        font-weight: 600;
      }

      .small {
        color: rgba(255, 255, 255, 0.7);
        font-size: 14px;
        line-height: 2rem;
        font-family: "UI Sans Serif", sans-serif;
      }
    }

    .config_list {
      width: 45%;
      height: 12vh;
      background-color: #353550;
      display: flex;
      flex-direction: column;
      gap: 10px; 
      justify-content: center;
      border: solid 1px rgba(114, 129, 225, 0.2);
      border-radius: 10px;

      .small {
        color: rgba(255, 255, 255, 0.7);
      }

      .small:first-child {
        text-align: center;
        font-size: 16px;
      }

      .small:last-child {
        text-align: left;
        font-size: 14px;
        padding-left: 20px; 
      }
    }

    .add_config {
      margin-top:40px ;
      width: 45%;
      height: 4vh; 
      background-color: #717CE0;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      border-radius: 5px;
      cursor: pointer;
    }
  }
}
</style>
