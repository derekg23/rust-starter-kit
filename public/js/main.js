async function fetchUsers() {
  try {
    const response = await fetch('/api/users');
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    const users = await response.json();

    const tbody = document.querySelector('#users tbody');
    tbody.innerHTML = ''; // Clear existing rows

    users.forEach(user => {
      const row = document.createElement('tr');
      row.dataset.id = user.id; // Store the user ID in the row

      row.innerHTML = `
        <td>${user.name}</td>
        <td>${user.email}</td>
        <td>
          <button class="edit-btn">Edit</button>
          <button class="delete-btn">Delete</button>
        </td>
      `;

      tbody.appendChild(row);
    });

    // Add event listeners for delete buttons
    document.querySelectorAll('.delete-btn').forEach(button => {
      button.addEventListener('click', async (event) => {
        const row = event.target.closest('tr');
        const userId = row.dataset.id;
        try {
          await fetch(`/api/users/${userId}`, {
            method: 'DELETE',
            headers: {
              'Content-Type': 'application/json',
            }
          });
          fetchUsers(); // Reload the users list
        } catch (error) {
          console.error('Error deleting user:', error);
        }
      });
    });

    // Add event listeners for edit buttons
    document.querySelectorAll('.edit-btn').forEach(button => {
      button.addEventListener('click', (event) => {
        const row = event.target.closest('tr');
        const userId = row.dataset.id;
        const name = row.querySelector('td:nth-child(1)').textContent;
        const email = row.querySelector('td:nth-child(2)').textContent;

        // Populate the edit modal fields with current user data
        document.getElementById('editName').value = name;
        document.getElementById('editEmail').value = email;

        // Show the edit modal
        const editModal = document.getElementById('editUserModal');
        document.getElementById('editUserError').style.display = 'none';
        editModal.style.display = 'block';

        // Handle form submission for editing
        const editForm = document.getElementById('editUserForm');
        editForm.onsubmit = async (e) => {
          e.preventDefault();

          const newName = document.getElementById('editName').value;
          const newEmail = document.getElementById('editEmail').value;

          try {
            const response = await fetch(`/api/users/${userId}`, {
              method: 'PUT',
              headers: {
                'Content-Type': 'application/json',
              },
              body: JSON.stringify({ name: newName, email: newEmail }),
            });

            if (response.ok) {
              editModal.style.display = 'none';
              fetchUsers(); // Reload the users list
            } else if (response.status === 409) { // Conflict status
              const message = await response.text();
              document.getElementById('editUserError').textContent = message;
              document.getElementById('editUserError').style.display = 'block'; // Show the error message
            } else {
              console.error('Error:', response.statusText);
            }
          } catch (error) {
            console.error('Error:', error);
          }
        };
      });
    });

  } catch (error) {
    console.error('Error fetching users:', error);
  }
}

document.addEventListener('DOMContentLoaded', () => {
  const createModal = document.getElementById('createUserModal');
  const editModal = document.getElementById('editUserModal');

  const openModalButton = document.getElementById('openModalButton');
  const closeCreateModalSpan = document.getElementsByClassName('close')[0];
  const closeEditModalSpan = document.getElementsByClassName('close')[1];
  const createForm = document.getElementById('createUserForm');
  
  // Open the create user modal
  openModalButton.onclick = () => {
    document.getElementById('createUserError').style.display = 'none';
    createModal.style.display = 'block';
  };
  
  // Close the modals
  closeCreateModalSpan.onclick = () => {
    createModal.style.display = 'none';
  };
  closeEditModalSpan.onclick = () => {
    editModal.style.display = 'none';
  };
  
  // Click outside of the modal to close it
  window.onclick = (event) => {
    if (event.target === createModal) {
      createModal.style.display = 'none';
    }
    if (event.target === editModal) {
      editModal.style.display = 'none';
    }
  };
  
  // Handle form submission for creating a new user
  createForm.onsubmit = async (event) => {
    event.preventDefault();
    
    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;

    try {
      const response = await fetch('/api/users', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ name, email }),
      });

      if (response.ok) {
        // Close the create modal
        createModal.style.display = 'none';

        // Clear error message
        document.getElementById('createUserError').textContent = '';

        // Reload the users table
        fetchUsers();
      } else if (response.status === 409) { // Conflict status
        const message = await response.text();
        document.getElementById('createUserError').textContent = message;
        document.getElementById('createUserError').style.display = 'block'; // Show the error message
      } else {
        console.error('Error:', response.statusText);
      }
    } catch (error) {
      console.error('Error:', error);
    }
  };
});

// Call the function to fetch and populate users when the page loads
window.onload = function() {
  fetchUsers();
};
