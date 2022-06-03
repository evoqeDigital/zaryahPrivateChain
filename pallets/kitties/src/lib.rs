#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement },
		transactional
	};
	use sp_io::hashing::blake2_128;
	use scale_info::TypeInfo;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding Gold information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Gold<T: Config> {
		pub id: [u8; 16],   // Using 16 bytes to represent id
		pub price: Option<BalanceOf<T>>,
		pub weight: u64,
		pub owner: AccountOf<T>,
	}

	// Struct for holding Silver information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Silver<T: Config> {
		pub id: [u8; 16],   // Using 16 bytes to represent id
		pub price: Option<BalanceOf<T>>,
		pub weight: u64,
		pub owner: AccountOf<T>,
	}

	// Struct for holding Land information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Land<T: Config> {
		pub id: [u8; 16],   // Using 16 bytes to represent id
		pub price: Option<BalanceOf<T>>,
		pub area: u64,
		pub owner: AccountOf<T>,
	}

	// Struct for holding Stock information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Stock<T: Config> {
		pub id: [u8; 16],   // Using 16 bytes to represent id
		pub price: Option<BalanceOf<T>>,
		pub nos: u64,
		pub owner: AccountOf<T>,
	}

	// Struct for holding Sukuk information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Sukuk<T: Config> {
		pub id: [u8; 16],   // Using 16 bytes to represent id
		pub price: Option<BalanceOf<T>>,
		pub owner: AccountOf<T>,
		pub rate: u64,
	}


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types it depends on.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of Gold a single account can own.
		#[pallet::constant]
		type MaxGoldOwned: Get<u32>;

		/// The maximum amount of Silver a single account can own.
		#[pallet::constant]
		type MaxSilverOwned: Get<u32>;

		/// The maximum amount of Land a single account can own
		#[pallet::constant]
		type MaxLandOwned: Get<u32>;

		/// The maximum amount of Stock a single account can own.
		#[pallet::constant]
		type MaxStockOwned: Get<u32>;

		/// The maximum amount of Sukuk a single account can own.
		#[pallet::constant]
		type MaxSukukOwned: Get<u32>;


		/// The type of Randomness we want to specify for this pallet.
		type UnitRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithmetic overflow when incrementing the Kitty counter.
		CountForUnitOverflow,
		/// An account cannot own more Kitties than `MaxKittyCount`.
		ExceedMaxUnitOwned,
		/// Buyer cannot be the owner.
		BuyerIsUnitOwner,
		/// Cannot transfer a kitty to its owner.
		TransferToSelf,
		/// This kitty already exists
		UnitExists,
		/// Handles checking whether the Kitty exists.
		UnitNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotUnitOwner,
		/// Ensures the Kitty is for sale.
		UnitNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		UnitBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,
	}

	// Events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new Unit was successfully created. \[sender, unit_id\]
		Created(T::AccountId, T::Hash),
		/// Unit price was successfully set. \[sender, unit_id, new_price\]
		PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		/// A Unit was successfully transferred. \[from, to, unit_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A Unit was successfully bought. \[buyer, seller, unit_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	// Storage items.

	#[pallet::storage]
	#[pallet::getter(fn count_for_gold)]
	/// Keeps track of the number of Gold in existence.
	pub(super) type CountForGold<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn count_for_silver)]
	/// Keeps track of the number of Silver in existence.
	pub(super) type CountForSilver<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn count_for_land)]
	/// Keeps track of the number of Land in existence.
	pub(super) type CountForLand<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn count_for_stock)]
	/// Keeps track of the number of Stock in existence.
	pub(super) type CountForStock<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn count_for_sukuk)]
	/// Keeps track of the number of Sukuk in existence.
	pub(super) type CountForSukuk<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn golds)]
	/// Stores a gold's unique traits, owner and price.
	pub(super) type Golds<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Gold<T>>;

	#[pallet::storage]
	#[pallet::getter(fn silvers)]
	/// Stores a silver's unique traits, owner and price.
	pub(super) type Silvers<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Silver<T>>;

	#[pallet::storage]
	#[pallet::getter(fn lands)]
	/// Stores a land's unique traits, owner and price.
	pub(super) type Lands<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Land<T>>;

	#[pallet::storage]
	#[pallet::getter(fn stocks)]
	/// Stores a stock's unique traits, owner and price.
	pub(super) type Stocks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Stock<T>>;

	#[pallet::storage]
	#[pallet::getter(fn sukuks)]
	/// Stores a sukuk's unique traits, owner and price.
	pub(super) type Sukuks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Sukuk<T>>;

	#[pallet::storage]
	#[pallet::getter(fn gold_owned)]
	/// Keeps track of what accounts own what Gold.
	pub(super) type GoldOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxGoldOwned>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn silver_owned)]
	/// Keeps track of what accounts own what Silver.
	pub(super) type SilverOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxSilverOwned>, ValueQuery>;	

	#[pallet::storage]
	#[pallet::getter(fn land_owned)]
	/// Keeps track of what account own what Lands.
	pub(super) type LandOwned<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxLandOwned>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn stock_owned)]
	/// Keeps track of what accounts own what Stock.
	pub(super) type StockOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxStockOwned>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sukuk_owned)]
	/// Keeps track of what accounts own what Sukuk.
	pub(super) type SukukOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxSukukOwned>, ValueQuery>;

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub golds: Vec<(T::AccountId, [u8; 16], u64)>,
		pub silvers: Vec<(T::AccountId, [u8; 16], u64)>,
		pub lands: Vec<(T::AccountId, [u8; 16], u64)>,
		pub stocks: Vec<(T::AccountId, [u8; 16], u64)>,
		pub sukuks: Vec<(T::AccountId, [u8; 16], u64)>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { 
				golds: vec![],
				silvers: vec![],
				lands: vec![],
				stocks: vec![],
				sukuks: vec![],
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a gold from genesis config, we require the id and weight to be supplied.
			for (acct, id, _weight) in &self.golds {
				let _ = <Pallet<T>>::mint_gold(acct, Some(id.clone()));
			}

			// When building a silver from genesis config, we require the id and weight to be supplied.
			for (acct, id, _weight) in &self.silvers {
				let _ = <Pallet<T>>::mint_silver(acct, Some(id.clone()));
			}

			// When building a land from genesis config, we require the id and area to be supplied.
			for (acct, id, _area) in &self.lands {
				let _ = <Pallet<T>>::mint_land(acct, Some(id.clone()));
			}

			// When building a stock from genesis config, we require the id and nos to be supplied.
			for (acct, id, _nos) in &self.stocks {
				let _ = <Pallet<T>>::mint_stock(acct, Some(id.clone()));
			}

			// When building a sukuk from genesis config, we require the id to be supplied.
			for (acct, id, _rate) in &self.sukuks {
				let _ = <Pallet<T>>::mint_sukuk(acct, Some(id.clone()));
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique gold.
		///
		/// The actual gold creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_gold(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let gold_id = Self::mint_gold(&sender, None)?;

			// Logging to the console
			let gold_coin_emoji = '\u{1FA99}';
			log::info!("{} A gold is minted with ID: {:?}.", gold_coin_emoji, gold_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, gold_id));
			Ok(())
		}

		/// Create a new unique silver.
		///
		/// The actual land creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_silver(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let silver_id = Self::mint_land(&sender, None)?;

			// Logging to the console
			log::info!("A silver is minted with ID: {:?}.", silver_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, silver_id));
			Ok(())
		}

		/// Create a new unique land.
		///
		/// The actual land creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_land(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let land_id = Self::mint_land(&sender, None)?;

			// Logging to the console
			log::info!("A land is minted with ID: {:?}.", land_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, land_id));
			Ok(())
		}
		
		/// Create a new unique stock.
		///
		/// The actual stock creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_stock(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let stock_id = Self::mint_stock(&sender, None)?;

			// Logging to the console
			log::info!("A stock is minted with ID: {:?}.", stock_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, stock_id));
			Ok(())
		}

		/// Create a new unique sukuk.
		///
		/// The actual sukuk creation is done in the `mint()` function.
		#[pallet::weight(100)]
		pub fn create_sukuk(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let sukuk_id = Self::mint_sukuk(&sender, None)?;

			// Logging to the console
			log::info!("A sukuk is minted with ID: {:?}.", sukuk_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created(sender, sukuk_id));
			Ok(())
		}		

		/// Set the price for a Gold.
		///
		/// Updates Gold price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price_gold(
			origin: OriginFor<T>,
			gold_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the gold exists and is called by the gold owner
			ensure!(Self::is_gold_owner(&gold_id, &sender)?, <Error<T>>::NotUnitOwner);

			let mut gold = Self::golds(&gold_id).ok_or(<Error<T>>::UnitNotExist)?;

			gold.price = new_price.clone();
			<Golds<T>>::insert(&gold_id, gold);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, gold_id, new_price));

			Ok(())
		}

		/// Set the price for a Silver.
		///
		/// Updates Silver price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price_silver(
			origin: OriginFor<T>,
			silver_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the silver exists and is called by the silver owner
			ensure!(Self::is_silver_owner(&silver_id, &sender)?, <Error<T>>::NotUnitOwner);

			let mut silver = Self::silvers(&silver_id).ok_or(<Error<T>>::UnitNotExist)?;

			silver.price = new_price.clone();
			<Silvers<T>>::insert(&silver_id, silver);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, silver_id, new_price));

			Ok(())
		}

		/// Set the price for a Land.
		///
		/// Updates Land price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price_land(
			origin: OriginFor<T>,
			land_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the land exists and is called by the land owner
			ensure!(Self::is_land_owner(&land_id, &sender)?, <Error<T>>::NotUnitOwner);

			let mut land = Self::lands(&land_id).ok_or(<Error<T>>::UnitNotExist)?;

			land.price = new_price.clone();
			<Lands<T>>::insert(&land_id, land);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, land_id, new_price));

			Ok(())
		}		

		/// Set the price for a Stock.
		///
		/// Updates Stock price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price_stock(
			origin: OriginFor<T>,
			stock_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the stock exists and is called by the stock owner
			ensure!(Self::is_stock_owner(&stock_id, &sender)?, <Error<T>>::NotUnitOwner);

			let mut stock = Self::stocks(&stock_id).ok_or(<Error<T>>::UnitNotExist)?;

			stock.price = new_price.clone();
			<Stocks<T>>::insert(&stock_id, stock);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, stock_id, new_price));

			Ok(())
		}

		/// Set the price for a Sukuk.
		///
		/// Updates Sukuk price and updates storage.
		#[pallet::weight(100)]
		pub fn set_price_sukuk(
			origin: OriginFor<T>,
			sukuk_id: T::Hash,
			new_price: Option<BalanceOf<T>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Ensure the sukuk exists and is called by the sukuk owner
			ensure!(Self::is_sukuk_owner(&sukuk_id, &sender)?, <Error<T>>::NotUnitOwner);

			let mut sukuk = Self::sukuks(&sukuk_id).ok_or(<Error<T>>::UnitNotExist)?;

			sukuk.price = new_price.clone();
			<Sukuks<T>>::insert(&sukuk_id, sukuk);

			// Deposit a "PriceSet" event.
			Self::deposit_event(Event::PriceSet(sender, sukuk_id, new_price));

			Ok(())
		}		

		/// Directly transfer a gold to another recipient.
		///
		/// Any account that holds a gold can send it to another Account. This will reset the asking
		/// price of the gold, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer_gold(
			origin: OriginFor<T>,
			to: T::AccountId,
			gold_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the gold exists and is called by the gold owner
			ensure!(Self::is_gold_owner(&gold_id, &from)?, <Error<T>>::NotUnitOwner);

			// Verify the gold is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more gold
			let to_owned = <GoldOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxGoldOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			Self::transfer_gold_to(&gold_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, gold_id));

			Ok(())
		}

		/// Directly transfer a silver to another recipient.
		///
		/// Any account that holds a silver can send it to another Account. This will reset the asking
		/// price of the silver, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer_silver(
			origin: OriginFor<T>,
			to: T::AccountId,
			silver_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the silver exists and is called by the silver owner
			ensure!(Self::is_silver_owner(&silver_id, &from)?, <Error<T>>::NotUnitOwner);

			// Verify the silver is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more silver
			let to_owned = <SilverOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxSilverOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			Self::transfer_silver_to(&silver_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, silver_id));

			Ok(())
		}

		/// Directly transfer a land to another recipient.
		///
		/// Any account that holds a land can send it to another Account. This will reset the asking
		/// price of the land, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer_land(
			origin: OriginFor<T>,
			to: T::AccountId,
			land_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the land exists and is called by the land owner
			ensure!(Self::is_land_owner(&land_id, &from)?, <Error<T>>::NotUnitOwner);

			// Verify the land is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more land
			let to_owned = <LandOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxLandOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			Self::transfer_land_to(&land_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, land_id));

			Ok(())
		}

		/// Directly transfer a stock to another recipient.
		///
		/// Any account that holds a stock can send it to another Account. This will reset the asking
		/// price of the stock, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer_stock(
			origin: OriginFor<T>,
			to: T::AccountId,
			stock_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the stock exists and is called by the stock owner
			ensure!(Self::is_stock_owner(&stock_id, &from)?, <Error<T>>::NotUnitOwner);

			// Verify the stock is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more stock
			let to_owned = <StockOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxStockOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			Self::transfer_stock_to(&stock_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, stock_id));

			Ok(())
		}

		/// Directly transfer a sukuk to another recipient.
		///
		/// Any account that holds a sukuk can send it to another Account. This will reset the asking
		/// price of the sukuk, marking it not for sale.
		#[pallet::weight(100)]
		pub fn transfer_sukuk(
			origin: OriginFor<T>,
			to: T::AccountId,
			sukuk_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the sukuk exists and is called by the sukuk owner
			ensure!(Self::is_sukuk_owner(&sukuk_id, &from)?, <Error<T>>::NotUnitOwner);

			// Verify the sukuk is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more sukuk
			let to_owned = <SukukOwned<T>>::get(&to);
			ensure!((to_owned.len() as u32) < T::MaxSukukOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			Self::transfer_gold_to(&sukuk_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, sukuk_id));

			Ok(())
		}

		/// Buy a saleable Gold. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the gold, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_gold(
			origin: OriginFor<T>,
			gold_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the gold exists and buyer is not the current gold owner
			let gold = Self::golds(&gold_id).ok_or(<Error<T>>::UnitNotExist)?;
			ensure!(gold.owner != buyer, <Error<T>>::BuyerIsUnitOwner);

			// Check the gold is for sale and the gold ask price <= bid_price
			if let Some(ask_price) = gold.price {
				ensure!(ask_price <= bid_price, <Error<T>>::UnitBidPriceTooLow);
			} else {
				Err(<Error<T>>::UnitNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more gold
			let to_owned = <GoldOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxGoldOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			let seller = gold.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the gold from seller to buyer
			Self::transfer_gold_to(&gold_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, gold_id, bid_price));

			Ok(())
		}

		/// Buy a saleable Silver. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the silver, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_silver(
			origin: OriginFor<T>,
			silver_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the silver exists and buyer is not the current silver owner
			let silver = Self::silvers(&silver_id).ok_or(<Error<T>>::UnitNotExist)?;
			ensure!(silver.owner != buyer, <Error<T>>::BuyerIsUnitOwner);

			// Check the silver is for sale and the silver ask price <= bid_price
			if let Some(ask_price) = silver.price {
				ensure!(ask_price <= bid_price, <Error<T>>::UnitBidPriceTooLow);
			} else {
				Err(<Error<T>>::UnitNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more silver
			let to_owned = <SilverOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxSilverOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			let seller = silver.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the silver from seller to buyer
			Self::transfer_silver_to(&silver_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, silver_id, bid_price));

			Ok(())
		}

		/// Buy a saleable Land. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the land, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_land(
			origin: OriginFor<T>,
			land_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the land exists and buyer is not the current land owner
			let land = Self::lands(&land_id).ok_or(<Error<T>>::UnitNotExist)?;
			ensure!(land.owner != buyer, <Error<T>>::BuyerIsUnitOwner);

			// Check the land is for sale and the land ask price <= bid_price
			if let Some(ask_price) = land.price {
				ensure!(ask_price <= bid_price, <Error<T>>::UnitBidPriceTooLow);
			} else {
				Err(<Error<T>>::UnitNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more land
			let to_owned = <GoldOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxGoldOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			let seller = land.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the land from seller to buyer
			Self::transfer_land_to(&land_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, land_id, bid_price));

			Ok(())
		}

		/// Buy a saleable Stock. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the stock, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_stock(
			origin: OriginFor<T>,
			stock_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the stock exists and buyer is not the current stock owner
			let stock = Self::stocks(&stock_id).ok_or(<Error<T>>::UnitNotExist)?;
			ensure!(stock.owner != buyer, <Error<T>>::BuyerIsUnitOwner);

			// Check the stock is for sale and the stock ask price <= bid_price
			if let Some(ask_price) = stock.price {
				ensure!(ask_price <= bid_price, <Error<T>>::UnitBidPriceTooLow);
			} else {
				Err(<Error<T>>::UnitNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more stock
			let to_owned = <StockOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxStockOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			let seller = stock.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the stock from seller to buyer
			Self::transfer_stock_to(&stock_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, stock_id, bid_price));

			Ok(())
		}
		
		/// Buy a saleable Sukuk. The bid price provided from the buyer has to be equal or higher
		/// than the ask price from the seller.
		///
		/// This will reset the asking price of the sukuk, marking it not for sale.
		/// Marking this method `transactional` so when an error is returned, we ensure no storage is changed.
		#[transactional]
		#[pallet::weight(100)]
		pub fn buy_sukuk(
			origin: OriginFor<T>,
			sukuk_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the sukuk exists and buyer is not the current sukuk owner
			let sukuk = Self::sukuks(&sukuk_id).ok_or(<Error<T>>::UnitNotExist)?;
			ensure!(sukuk.owner != buyer, <Error<T>>::BuyerIsUnitOwner);

			// Check the sukuk is for sale and the sukuk ask price <= bid_price
			if let Some(ask_price) = sukuk.price {
				ensure!(ask_price <= bid_price, <Error<T>>::UnitBidPriceTooLow);
			} else {
				Err(<Error<T>>::UnitNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more sukuk
			let to_owned = <SukukOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxSukukOwned::get(), <Error<T>>::ExceedMaxUnitOwned);

			let seller = sukuk.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the sukuk from seller to buyer
			Self::transfer_sukuk_to(&sukuk_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, sukuk_id, bid_price));

			Ok(())
		}

	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {

		fn gen_id() -> [u8; 16] {
			let payload = (
				T::UnitRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::extrinsic_index().unwrap_or_default(),
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		// Helper to mint a Gold.
		pub fn mint_gold(
			owner: &T::AccountId,
			id: Option<[u8; 16]>,
		) -> Result<T::Hash, Error<T>> {
			let gold = Gold::<T> {
				id: id.unwrap_or_else(Self::gen_id),
				price: None,
				weight: 10,
				owner: owner.clone(),
			};

			let gold_id = T::Hashing::hash_of(&gold);

			// Performs this operation first as it may fail
			let new_cnt = Self::count_for_gold().checked_add(1)
				.ok_or(<Error<T>>::CountForUnitOverflow)?;

			// Check if the Gold does not already exist in our storage map
			ensure!(Self::golds(&gold_id) == None, <Error<T>>::UnitExists);

			// Performs this operation first because as it may fail
			<GoldOwned<T>>::try_mutate(&owner, |gold_vec| {
				gold_vec.try_push(gold_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			<Golds<T>>::insert(gold_id, gold);
			<CountForGold<T>>::put(new_cnt);
			Ok(gold_id)
		}

		// Helper to mint a Silver.
		pub fn mint_silver(
			owner: &T::AccountId,
			id: Option<[u8; 16]>,
		) -> Result<T::Hash, Error<T>> {
			let silver = Silver::<T> {
				id: id.unwrap_or_else(Self::gen_id),
				price: None,
				weight: 10,
				owner: owner.clone(),
			};

			let silver_id = T::Hashing::hash_of(&silver);

			// Performs this operation first as it may fail
			let new_cnt = Self::count_for_silver().checked_add(1)
				.ok_or(<Error<T>>::CountForUnitOverflow)?;

			// Check if the Silver does not already exist in our storage map
			ensure!(Self::silvers(&silver_id) == None, <Error<T>>::UnitExists);

			// Performs this operation first because as it may fail
			<SilverOwned<T>>::try_mutate(&owner, |silver_vec| {
				silver_vec.try_push(silver_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			<Silvers<T>>::insert(silver_id, silver);
			<CountForSilver<T>>::put(new_cnt);
			Ok(silver_id)
		}

		// Helper to mint a Land.
		pub fn mint_land(
			owner: &T::AccountId,
			id: Option<[u8; 16]>,
		) -> Result<T::Hash, Error<T>> {
			let land = Land::<T> {
				id: id.unwrap_or_else(Self::gen_id),
				price: None,
				area: 10,
				owner: owner.clone(),
			};

			let land_id = T::Hashing::hash_of(&land);

			// Performs this operation first as it may fail
			let new_cnt = Self::count_for_land().checked_add(1)
				.ok_or(<Error<T>>::CountForUnitOverflow)?;

			// Check if the Land does not already exist in our storage map
			ensure!(Self::lands(&land_id) == None, <Error<T>>::UnitExists);

			// Performs this operation first because as it may fail
			<GoldOwned<T>>::try_mutate(&owner, |land_vec| {
				land_vec.try_push(land_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			<Lands<T>>::insert(land_id, land);
			<CountForLand<T>>::put(new_cnt);
			Ok(land_id)
		}

		// Helper to mint a Stock.
		pub fn mint_stock(
			owner: &T::AccountId,
			id: Option<[u8; 16]>,
		) -> Result<T::Hash, Error<T>> {
			let stock = Stock::<T> {
				id: id.unwrap_or_else(Self::gen_id),
				price: None,
				nos: 10,
				owner: owner.clone(),
			};

			let stock_id = T::Hashing::hash_of(&stock);

			// Performs this operation first as it may fail
			let new_cnt = Self::count_for_stock().checked_add(1)
				.ok_or(<Error<T>>::CountForUnitOverflow)?;

			// Check if the Stock does not already exist in our storage map
			ensure!(Self::stocks(&stock_id) == None, <Error<T>>::UnitExists);

			// Performs this operation first because as it may fail
			<StockOwned<T>>::try_mutate(&owner, |stock_vec| {
				stock_vec.try_push(stock_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			<Stocks<T>>::insert(stock_id, stock);
			<CountForStock<T>>::put(new_cnt);
			Ok(stock_id)
		}

		// Helper to mint a Sukuk.
		pub fn mint_sukuk(
			owner: &T::AccountId,
			id: Option<[u8; 16]>,
		) -> Result<T::Hash, Error<T>> {
			let sukuk = Sukuk::<T> {
				id: id.unwrap_or_else(Self::gen_id),
				price: None,
				rate: 3,
				owner: owner.clone(),
			};

			let sukuk_id = T::Hashing::hash_of(&sukuk);

			// Performs this operation first as it may fail
			let new_cnt = Self::count_for_sukuk().checked_add(1)
				.ok_or(<Error<T>>::CountForUnitOverflow)?;

			// Check if the Sukuk does not already exist in our storage map
			ensure!(Self::sukuks(&sukuk_id) == None, <Error<T>>::UnitExists);

			// Performs this operation first because as it may fail
			<SukukOwned<T>>::try_mutate(&owner, |sukuk_vec| {
				sukuk_vec.try_push(sukuk_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			<Sukuks<T>>::insert(sukuk_id, sukuk);
			<CountForSukuk<T>>::put(new_cnt);
			Ok(sukuk_id)
		}

		// Check if account is owner of gold
		pub fn is_gold_owner(gold_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::golds(gold_id) {
				Some(gold) => Ok(gold.owner == *acct),
				None => Err(<Error<T>>::UnitNotExist)
			}
		}

		// Check if account is owner of silver
		pub fn is_silver_owner(silver_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::silvers(silver_id) {
				Some(silver) => Ok(silver.owner == *acct),
				None => Err(<Error<T>>::UnitNotExist)
			}
		}

		// Check if account is owner of land
		pub fn is_land_owner(land_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::lands(land_id) {
				Some(land) => Ok(land.owner == *acct),
				None => Err(<Error<T>>::UnitNotExist)
			}
		}

		// Check if account is owner of stock
		pub fn is_stock_owner(stock_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::stocks(stock_id) {
				Some(stock) => Ok(stock.owner == *acct),
				None => Err(<Error<T>>::UnitNotExist)
			}
		}

		// Check if account is owner of sukuk
		pub fn is_sukuk_owner(sukuk_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::sukuks(sukuk_id) {
				Some(sukuk) => Ok(sukuk.owner == *acct),
				None => Err(<Error<T>>::UnitNotExist)
			}
		}

		// Transfer Gold
		#[transactional]
		pub fn transfer_gold_to(
			gold_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut gold = Self::golds(&gold_id).ok_or(<Error<T>>::UnitNotExist)?;

			let prev_owner = gold.owner.clone();

			// Remove `gold_id` from the GoldOwned vector of `prev_gold_owner`
			<GoldOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *gold_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::UnitNotExist)?;

			// Update the gold owner
			gold.owner = to.clone();
			// Reset the ask price so the gold is not for sale until `set_price()` is called
			// by the current owner.
			gold.price = None;

			<Golds<T>>::insert(gold_id, gold);

			<GoldOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*gold_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			Ok(())
		}

		// Transfer Silver
		#[transactional]
		pub fn transfer_silver_to(
			silver_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut silver = Self::silvers(&silver_id).ok_or(<Error<T>>::UnitNotExist)?;

			let prev_owner = silver.owner.clone();

			// Remove `silver_id` from the SilverOwned vector of `prev_silver_owner`
			<SilverOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *silver_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::UnitNotExist)?;

			// Update the silver owner
			silver.owner = to.clone();
			// Reset the ask price so the silver is not for sale until `set_price()` is called
			// by the current owner.
			silver.price = None;

			<Silvers<T>>::insert(silver_id, silver);

			<SilverOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*silver_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			Ok(())
		}

		// Transnfer Land
		#[transactional]
		pub fn transfer_land_to(
			land_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut land = Self::lands(&land_id).ok_or(<Error<T>>::UnitNotExist)?;

			let prev_owner = land.owner.clone();

			// Remove `land_id` from the LandOwned vector of `prev_land_owner`
			<LandOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *land_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::UnitNotExist)?;

			// Update the land owner
			land.owner = to.clone();
			// Reset the ask price so the land is not for sale until `set_price()` is called
			// by the current owner.
			land.price = None;

			<Lands<T>>::insert(land_id, land);

			<LandOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*land_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			Ok(())
		}		

		// Transfer Stock
		#[transactional]
		pub fn transfer_stock_to(
			stock_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut stock = Self::stocks(&stock_id).ok_or(<Error<T>>::UnitNotExist)?;

			let prev_owner = stock.owner.clone();

			// Remove `stock_id` from the StockOwned vector of `prev_stock_owner`
			<StockOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *stock_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::UnitNotExist)?;

			// Update the stock owner
			stock.owner = to.clone();
			// Reset the ask price so the stock is not for sale until `set_price()` is called
			// by the current owner.
			stock.price = None;

			<Stocks<T>>::insert(stock_id, stock);

			<StockOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*stock_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			Ok(())
		}

		// Transfer Sukuk
		#[transactional]
		pub fn transfer_sukuk_to(
			sukuk_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut sukuk = Self::sukuks(&sukuk_id).ok_or(<Error<T>>::UnitNotExist)?;

			let prev_owner = sukuk.owner.clone();

			// Remove `sukuk_id` from the SukukOwned vector of `prev_sukuk_owner`
			<SukukOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *sukuk_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::UnitNotExist)?;

			// Update the sukuk owner
			sukuk.owner = to.clone();
			// Reset the ask price so the gold is not for sale until `set_price()` is called
			// by the current owner.
			sukuk.price = None;

			<Sukuks<T>>::insert(sukuk_id, sukuk);

			<SukukOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*sukuk_id)
			}).map_err(|_| <Error<T>>::ExceedMaxUnitOwned)?;

			Ok(())
		}
	}
}
