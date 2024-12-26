import { promises as fs } from 'fs';

type Body = {
  username: string;
  password: string;
};

type User = {
  username: string;
  password: string;
}

async function readUsersFile(): Promise<User[]> {
  try {
    const data = await fs.readFile('users.json', 'utf-8');
    const users = JSON.parse(data) as User[];

    return users;
  } catch (error) {
    throw new Error('Failed to read users.json file');
  }
};

async function writeUsersFile(users: User[]): Promise<void> {
  try {
    await fs.writeFile('users.json', JSON.stringify(users));
  } catch (error) {
    throw new Error('Failed to write users.json file');
  }
};

export default defineEventHandler(async (event) => {
  const { username, password }: Body = await readBody(event);
  const users = await readUsersFile();

  /**
   * this is very insecure! in production systems, you should have your passwords and use a real database!
   */
  const user = users.find(u => u.username === username);

  // if the user doesn't exist, add the user.
  if (!user) {

    // add the user to the list
    users.push({
      username,
      password
    });

    await writeUsersFile(users);

    return {
      success: true,
      action: 'register'
    }
  }

  // if the user exists, check the password
  if (user.password !== password) {
    return {
      success: false,
      message: 'Invalid password'
    }
  }
  
  return {
    success: true,
    action: 'login'
  }
});
