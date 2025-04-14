<script setup>
import { ref } from 'vue';
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

const emit = defineEmits(['file-uploaded']);
const fileContent = ref('');

const handleFileDrop = async (event) => {
  const file = event.dataTransfer.files[0];
  fileContent.value = await file.text();
  emit('file-uploaded', fileContent.value);
};
</script>

<template>
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
</template>

<style scoped lang="scss">
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
</style>
