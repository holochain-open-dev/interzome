import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  TransportConfigType,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen({});

// Construct proper paths for your DNAs
const interzome = path.join(__dirname, "../../interzome.dna.gz");

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [interzome],
  ],
  [
    // happ 0
    [interzome],
  ],
];

orchestrator.registerScenario(
  "create a user entry and find agent_key by username from another zome",
  async (s, t) => {
    const [player]: Player[] = await s.players([conductorConfig]);
    const [[alice_happ]] = await player.installAgentsHapps(
      installation
    );

    const alice_ = alice_happ.cells[0];

    let agent_data = await alice_.call(
      "zomeA",
      "set_username",
      {
       user: "Alice"
      }
    );
    console.log(agent_data)
    t.ok(agent_data);

    await sleep(200);

    let pubkey = await alice_.call(
      "zomeB",
      "get_agent_pubkey_from_username",
      "Alice"
    );
    t.equal(pubkey,agent_data.user_pub_key);
  }
);

orchestrator.run();
