library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity Uart is
  generic (
    -- This assumes a 25 Mhz clock, with 115_200 Bd.
    CLOCKS_PER_BIT : positive := 217
  );
  port (
    clk : in std_logic;
    -- Incoming serial line.
    rx : in std_logic;
    -- 1 for a single clock cycle after we finish reading one byte of data.
    data_valid : out std_logic;
    -- The byte of data.
    data : out std_logic_vector(7 downto 0)
  );
end Uart;

architecture bhv of Uart is
  -- We have a bit of delay in reading `rx` for stability.
  signal rx_sync : std_logic_vector(1 downto 0);
  -- Used for edge comparison.
  signal rx_prev : std_logic;

  type StateT is (SIdle, SStart, SReading, SSTop);

  signal state : StateT := SIdle;

  signal counter : natural range 0 to CLOCKS_PER_BIT - 1 := 0;

  signal data_reg : std_logic_vector(7 downto 0) := (others => '0');

  signal data_counter : natural range 0 to 7 := 0;
begin
  data <= data_reg;

  process(clk)
  begin
    if rising_edge(clk) then
      rx_sync(1) <= rx_sync(0);
      rx_sync(0) <= rx;
    end if;
  end process;

  process(clk)
  begin
    if rising_edge(clk) then
      data_valid <= '0';
      rx_prev <= rx_sync(1);
      if counter /= 0 then
        counter <= counter - 1;
      end if;
      case state is
        when SIdle =>
          if rx_prev = '1' and rx_sync(1) = '0' then
            state <= SStart;
            counter <= CLOCKS_PER_BIT / 2 - 1;
          end if;
        when SStart =>
          if counter = 0 then
            if rx_sync(1) = '0' then
              state <= SReading;
              counter <= CLOCKS_PER_BIT - 1;
              data_counter <= 7;
            else
              -- False start.
              state <= SIdle;
            end if;
          end if;
        when SReading =>
          if counter = 0 then
            data_reg <= rx_sync(1) & data_reg(7 downto 1);
            if data_counter /= 0 then
              data_counter <= data_counter - 1;
              counter <= CLOCKS_PER_BIT - 1;
            else
              state <= SSTop;
              counter <= CLOCKS_PER_BIT - 1;
            end if;
          end if;
        when SSTop =>
          if counter = 0 then
            if rx_sync(1) = '1' then
              data_valid <= '1';
            end if;
            state <= SIdle;
          end if;
      end case;
    end if;
  end process;
end bhv;
