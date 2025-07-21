-- migrate:up

DO $$
BEGIN
	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'ed_addr'
	) THEN
		CREATE DOMAIN ED_ADDR AS CHAR(66);
	END IF;

	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'ed_sig'
	) THEN
		CREATE DOMAIN ED_SIG AS NUMERIC(130, 0);
	END IF;

	IF NOT EXISTS (
		SELECT 1 FROM pg_type WHERE typname = 'hugeint'
	) THEN
		CREATE DOMAIN HUGEINT AS NUMERIC(78, 0);
	END IF;
END $$;

CREATE TABLE valleygame_game_info_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	round_limit INTEGER NOT NULL,
	bip_points INTEGER NOT NULL,
	fixed_buyin HUGEINT NOT NULL,
	whitelisted_ids JSONB NOT NULL,
	-- This is also used externally to convert to this game's id.
	random_nonce HUGEINT NOT NULL
);

-- Server-side deposits that were created from a centralised source.
CREATE TABLE valleygame_server_deposits_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	recipient ED_ADDR NOT NULL,
	amount HUGEINT NOT NULL,
	nonce INTEGER NOT NULL,
	sig ED_SIG NOT NULL
);

CREATE TABLE valleygame_mint_debt_token_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	from_deposit_ticket HUGEINT,
	from_previous_game_id SERIAL,
	from_server_id SERIAL,
	signer ED_ADDR NOT NULL,
	sig ED_SIG NOT NULL,
	FOREIGN KEY (from_previous_game_id) REFERENCES valleygame_game_info_1(id),
	FOREIGN KEY (from_server_id) REFERENCES valleygame_server_deposits_1(id)
);

CREATE TABLE valleygame_enter_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	-- We get the args from the game info when someone asks for this.
	game_id SERIAL NOT NULL,
	FOREIGN KEY (game_id) REFERENCES valleygame_game_info_1(id)
);

CREATE TABLE valleygame_prediction_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	game_id SERIAL NOT NULL,
	estimated HUGEINT NOT NULL,
	round_number INTEGER NOT NULL,
	signer ED_ADDR NOT NULL,
	sig ED_SIG NOT NULL,
	FOREIGN KEY (game_id) REFERENCES valleygame_game_info_1(id)
);

CREATE TABLE valleygame_withdraw_1 (
	id SERIAL PRIMARY KEY,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	debt_token_id SERIAL NOT NULL,
	user_signer ED_ADDR NOT NULL,
	user_sig ED_SIG NOT NULL,
	server_sig ED_SIG NOT NULL,
	FOREIGN KEY (debt_token_id) REFERENCES valleygame_mint_debt_token_1(id)
);

-- migrate:down
