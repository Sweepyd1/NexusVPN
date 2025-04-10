<script setup>
import { ref } from 'vue';

const configName = ref('');
const selectedProtocol = ref('');
const isDragging = ref(false);

const handleDragOver = (e) => {
  e.preventDefault();
  isDragging.value = true;
};

const handleDragLeave = () => {
  isDragging.value = false;
};

const handleDrop = (e) => {
  e.preventDefault();
  isDragging.value = false;
  const files = e.dataTransfer.files;
  console.log(files)

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
        <select v-model="selectedProtocol" class="select-field">
      
          <option value="wireguard">WireGuard</option>
          <option value="openvpn">OpenVPN</option>
          <option value="vless">VLESS</option>
        </select>
      </div>

      <div 
        class="drop-zone"
        :class="{ 'dragging': isDragging }"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
      >
        <div class="drop-content">
          <svg class="upload-icon" viewBox="0 0 24 24">
            <path d="M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z"/>
          </svg>
          <span>Drag and drop config file or click to upload</span>
        </div>
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

    .drop-zone {
      flex: 1;
      border: 2px dashed #404060;
      border-radius: 8px;
      padding: 2rem;
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.3s;
      background-color: rgba(64, 64, 96, 0.1);
      cursor: pointer;

      &.dragging {
        border-color: #6464FF;
        background-color: rgba(100, 100, 255, 0.1);
      }

      .drop-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        color: #A0A0C0;
        text-align: center;

        .upload-icon {
          width: 48px;
          height: 48px;
          fill: #6464FF;
          opacity: 0.8;
        }
      }
    }
  }
}
</style>