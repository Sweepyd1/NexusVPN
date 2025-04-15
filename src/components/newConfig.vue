<script setup>
import { ref } from 'vue';
import drag_and_drop from './drag_and_drop.vue';
import { invoke } from '@tauri-apps/api/core';
const configName = ref('');
const selectedProtocol = ref('wireguard');
const vless_data = ref('')

const is_show_drag_and_drop = ref(true)

const handleFileUpload = (content) => {
  fileContent.value = content;
};

const handleProtocol = () => {
  if (selectedProtocol.value === "openvpn"){
    is_show_drag_and_drop.value = true
  } 
  else if (selectedProtocol.value === "wireguard"){
    is_show_drag_and_drop.value = true
  } 
  else if (selectedProtocol.value === "vless"){
    is_show_drag_and_drop.value = false
  } 

}
const create_nexus_dir = async () => {
  console.log('Папка .nexus создана!');
  try {
    await invoke('create_nexus_dir');
    console.log('Папка .nexus создана!');
  } catch (err) {
    console.error('Ошибка:', err);
  }
};

const saveConfig = async () => {
  if (selectedProtocol.value === 'vless') {
    if (!configName.value || !vless_data.value) {
      alert('Заполните все поля!');
      return;
    }
    
    try {
      await invoke('save_vless_config', {
        name: configName.value,
        vlessData: vless_data.value
      });
      console.log('Конфиг сохранен!');
      emit('close');
    } catch (err) {
      console.error('Ошибка:', err);
    }
  } else {
 
    create_nexus_dir();
  }
};
const startVPN = async () => {
  if (!configName.value) {
    alert('Выберите конфигурацию!');
    return;
  }
  
  try {
    const result = await invoke('start_vless', { configName: configName.value });
    console.log(result);
    alert('VPN запущен!');
  } catch (err) {
    console.error(err);
    alert('Ошибка: ' + err);
  }
};

</script>

<template>
  <div class="container" @click.self="$emit('close')">
    <div class="modal">
      <span class="title">Add VPN Configuration</span>
      
      <div class="form-group">
        <label>Configuration Name</label>
        <input 
          v-model="configName" 
          type="text" 
          placeholder="My VPN Config" 
          class="input-field"
        >
      </div>

      <div class="form-group">
        <label>Protocol</label>
        <select v-model="selectedProtocol" class="select-field" @change="handleProtocol">
      
          <option value="wireguard">WireGuard</option>
          <option value="openvpn">OpenVPN</option>
          <option value="vless">VLESS</option>
        </select>

       
      </div>
      <div class="form-group" v-if="!is_show_drag_and_drop">
        <label >VLESS data</label>
      <input 
          v-model="vless_data" 
          type="text" 
          placeholder="vless" 
          class="input-field"
        >

        
      </div>
      <drag_and_drop v-if="is_show_drag_and_drop"  @file-uploaded="handleFileUpload"/>
      
      <div class="save_config" @click="saveConfig">
        <span>Save configuration</span>
      </div>

    </div>
  </div>

    



</template>

<style scoped lang="scss">
.container {
  position: absolute;
  width: 100%;
  height: 100vh;
  background-color: rgba(9, 9, 13, 0.9); 
  z-index: 10;
  display: flex;
  align-items: center;
  justify-content: center;

  .modal {
    width: 30%;
    min-width: 400px;
    height: 80vh;
    background-color: #353550;
    padding: 25px;
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    .title {
      color: white;
      font-size: 1.5rem;
      font-weight: 600;
      margin-bottom: 1rem;
      display: block;
    }

    .form-group {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      label {
        color: #A0A0C0;
        font-size: 0.9rem;
      }

      .input-field, .select-field {
        background: #2A2A40;
        border: 1px solid #404060;
        border-radius: 6px;
        padding: 0.75rem;
        color: white;
        font-size: 1rem;
        transition: border-color 0.3s;

        &:focus {
          outline: none;
          border-color: #6464FF;
        }
      }

      .select-field {
        appearance: none;
        background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='white'%3e%3cpath d='M7 10l5 5 5-5z'/%3e%3c/svg%3e");
        background-repeat: no-repeat;
        background-position: right 0.75rem center;
        background-size: 1.2em;
      }
    }

    .save_config {
      margin-top:40px ;
      width: 100%;
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