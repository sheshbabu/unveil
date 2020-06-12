import * as React from "react";
import {
  Modal,
  Form,
  Container,
  Divider,
  Loader,
  Menu,
  Header,
  Button,
  Table,
  Checkbox,
  CheckboxProps,
} from "semantic-ui-react";
import request from "./request";

type Flag = {
  id: string;
  name: string;
  is_enabled: boolean;
};

export default function App() {
  const [flags, setFlags] = React.useState<Flag[]>([]);
  const [isLoading, setIsLoading] = React.useState<boolean>(true);
  const [isModalOpen, setIsModalOpen] = React.useState<boolean>(false);

  let content = <FlagsTable flags={flags} onAddFlagClick={handleAddFlagClick} onFlagToggle={handleFlagToggle} />;

  React.useEffect(() => {
    fetchFlags();
  }, []);

  async function fetchFlags() {
    try {
      const flags = await request("/api/flags");
      setFlags(flags);
    } finally {
      setIsLoading(false);
    }
  }

  function handleAddFlagClick() {
    setIsModalOpen(true);
  }

  async function handleSaveFlagClick() {
    setIsModalOpen(false);
    await fetchFlags();
  }

  async function handleFlagToggle() {
    await fetchFlags();
  }

  function handleModalClose() {
    setIsModalOpen(false);
  }

  if (flags.length === 0) {
    content = (
      <div className="vx-empty-content">
        <Header as="h2" textAlign="center">
          Welcome!
        </Header>
        <p>Get started by creating your first flag.</p>
        <Button primary onClick={handleAddFlagClick}>
          Add Flag
        </Button>
      </div>
    );
  }

  if (isLoading) {
    content = (
      <Loader active inline="centered">
        Loading
      </Loader>
    );
  }

  return (
    <Container text>
      <Menu attached="bottom">
        <Menu.Item header>Vexil</Menu.Item>
      </Menu>
      <Divider hidden />
      {content}
      <AddFlagModal isOpen={isModalOpen} onSaveFlagClick={handleSaveFlagClick} onClose={handleModalClose} />
    </Container>
  );
}

type FlagsTableProps = {
  flags: Flag[];
  onAddFlagClick: () => void;
  onFlagToggle: () => void;
};

function FlagsTable(props: FlagsTableProps) {
  async function handleFlagToggle(data: CheckboxProps, flagName: string) {
    const name = flagName;
    const is_enabled = data.checked;
    await request("/api/flags", "PUT", { name, is_enabled });
    props.onFlagToggle();
  }

  const rows = props.flags.map((flag) => {
    return (
      <Table.Row>
        <Table.Cell>{flag.name}</Table.Cell>
        <Table.Cell>
          <Checkbox toggle checked={flag.is_enabled} onChange={(e, data) => handleFlagToggle(data, flag.name)} />
        </Table.Cell>
      </Table.Row>
    );
  });

  return (
    <div>
      <div className="vx-content">
        <Button primary onClick={props.onAddFlagClick}>
          Add Flag
        </Button>
      </div>
      <Table celled striped singleLine compact size="small">
        <Table.Header>
          <Table.Row>
            <Table.HeaderCell width={9}>Name</Table.HeaderCell>
            <Table.HeaderCell width={1}>Status</Table.HeaderCell>
          </Table.Row>
        </Table.Header>
        <tbody>{rows}</tbody>
      </Table>
    </div>
  );
}

type AddFlagModalProps = {
  isOpen: boolean;
  onSaveFlagClick: () => void;
  onClose: () => void;
};

function AddFlagModal(props: AddFlagModalProps) {
  const [isLoading, setIsLoading] = React.useState<boolean>(false);
  const [name, setName] = React.useState<string>("");

  React.useEffect(() => {
    setName("");
    setIsLoading(false);
  }, [props.isOpen]);

  async function handleSaveFlagClick() {
    setIsLoading(true);
    await request("/api/flags", "POST", { name });
    setIsLoading(false);
    props.onSaveFlagClick();
  }

  return (
    <Modal size="tiny" open={props.isOpen} onClose={props.onClose}>
      <Modal.Header>Add Flag</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Field>
            <label>Name</label>
            <input placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
          </Form.Field>
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button color="blue" onClick={handleSaveFlagClick} loading={isLoading}>
          Save
        </Button>
      </Modal.Actions>
    </Modal>
  );
}
