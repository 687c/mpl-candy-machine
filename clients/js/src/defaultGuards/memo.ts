import { PublicKey } from '@metaplex-foundation/umi';
import { Memo, MemoArgs, getMemoSerializer } from '../generated';
import { GuardManifest, noopParser } from '../guards';

export const memoGuardManifest: GuardManifest<
  MemoArgs,
  Memo,
  MemoGuardMintArgs
> = {
  name: 'memo',
  serializer: getMemoSerializer,
  mintParser: (context, mintContext, args) => {
    return {
      data: new Uint8Array(),
      // Pass in any accounts needed for your custom guard from your mint args.
      // Your guard may or may not need remaining accounts.
      remainingAccounts: [{ publicKey: args.minter, isWritable: true }],
    };
  },
  routeParser: noopParser,
};

export type MemoGuardMintArgs = {
  minter: PublicKey;
};
