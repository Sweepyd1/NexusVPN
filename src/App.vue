<script setup>
import ProtectIcon from "./svg/protect.vue";
import NoProtectionIcon from "./svg/noprotect.vue";
import { invoke } from '@tauri-apps/api/core';
import newConfig from "./components/newConfig.vue";
import { ref, onMounted } from "vue";

import vpnConfig from "./components/vpnConfig.vue";


onMounted(async () => {
  try {
    configs.value = await invoke('get_configs');
 
  } catch (err) {
    console.error('Ошибка загрузки конфигов:', err);
  }
});

const configs = ref([]);

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
      <span>Nexus VPN</span>
    </div>

    <div class="main">
      <div class="main_block">
  <div class="status-card">
    <div class="status-content">
      <span class="status-title">Not Protected</span>
      <NoProtectionIcon class="status-icon"></NoProtectionIcon>
      <span class="status-subtitle">Not connected</span>
    </div>
  </div>

        <div class="configs-wrapper">
          <div class="empty-config" v-if="configs.length == 0">
            <span class="empty-title">No VPN configurations yet</span>
            <span class="empty-subtitle">Add a new configuration to get started</span>
          </div>
            <vpnConfig v-for="config in configs" :key="config.name" :name="config.name" :data="config.vless_data"/>
          <button class="add-config-btn" @click="openModal">
        Add New Configuration
      </button>



        </div>
      </div>

     
    </div>
  </div>
</template>

<style lang="scss">
body {
  margin: 0;
  padding: 0;
}
.container {
  min-height: 100vh;
  padding: 1.5rem 1rem;
  background-color: #2c2c44;
  color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 2rem;
  
  span {
    font-size: 1.25rem;
    font-weight: 600;
    
    @media (min-width: 768px) {
      font-size: 1.5rem;
    }
  }
}

.main {
  width: 100%;
  max-width: 800px;
}

.main_block {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  
  
  @media (min-width: 768px) {
    flex-direction: row;
  }
}

.status-card {
  flex: 0 0 auto;
  height: fit-content;
  min-height: 160px;
  background: #353550;
  border: 1px solid rgba(114, 129, 225, 0.2);
  border-radius: 0.75rem;
  padding: 1.5rem; 


  @media (min-width: 769px) {
    width: 35%; 
    max-width: 400px; 
    min-width: 300px;
  }

  @media (max-width: 768px) {
    width: 90%; 

  }


  @media (min-width: 481px) and (max-width: 768px) {
    width: 60%; 
    margin: 0 auto 1rem;
  }


 
  
  .status-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    text-align: center;
  }
  
  .status-title {
    font-size: 1.25rem;
    font-weight: 600;
  }
  
  .status-subtitle {
    color: rgba(255, 255, 255, 0.7);
    font-size: 0.875rem;
  }
}

.configs-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-width: 0; 
}

.empty-config {
  background: #353550;
  border: 1px solid rgba(114, 129, 225, 0.2);
  border-radius: 0.75rem;
  padding: 2.5rem;
  text-align: center;
  
  .empty-title {
    display: block;
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }
  
  .empty-subtitle {
    color: rgba(255, 255, 255, 0.7);
    font-size: 0.875rem;
    line-height: 1.25;
  }
}



.connect-btn {
  background: transparent;
  border: 1px solid #717CE0;
  color: #717CE0;
  padding: 0.375rem 0.75rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  cursor: pointer;
  transition: opacity 0.2s;
  
  &:hover {
    opacity: 0.8;
  }
}

.delete-btn {
  background: transparent;
  border: none;
  color: rgba(255, 255, 255, 0.7);
  padding: 0.25rem;
  cursor: pointer;
  
  &:hover {
    color: white;
  }
}

.add-config-btn {
  width: 100%;

  margin: 2rem auto 0;
  padding: 0.75rem 1.5rem;
  background: #717CE0;
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 1rem;
  cursor: pointer;
  transition: opacity 0.2s;
  display: block;
  
  &:hover {
    opacity: 0.9;
  }
}

@media (max-width: 640px) {
  .main_block {
    gap: 1rem;
  }
  
  .status-card {
    padding: 1rem;
    
    .status-title {
      font-size: 1.1rem;
    }
  }
  
  .vpn-config {
    padding: 0.75rem;
    gap: 0.75rem;
    
    .config-actions {
      flex-direction: column;
    }
    
    .connect-btn {
      padding: 0.25rem 0.5rem;
    }
  }
  
  .add-config-btn {
    margin-top: 1.5rem;
    padding: 0.625rem 1rem;
    font-size: 0.875rem;
  }
}
</style>
