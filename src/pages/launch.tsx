import {
  Box,
  Button,
  Card,
  Center,
  HStack,
  IconButton,
  IconButtonProps,
  Image,
  Text,
  Tooltip,
  VStack,
  useColorModeValue,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { LuArrowLeftRight } from "react-icons/lu";
import { useData } from "@/contexts/data";
import { useThemedCSSStyle } from "@/hooks/themed-css";
import { Player } from "@/models/account";
import { GameInstanceSummary } from "@/models/instance";
import styles from "@/styles/launch.module.css";
import { base64ImgSrc } from "@/utils/string";

interface SwitchButtonProps extends IconButtonProps {
  tooltip: string;
}

const SwitchButton: React.FC<SwitchButtonProps> = ({ tooltip, ...props }) => {
  return (
    <Tooltip label={tooltip} placement="top-end">
      <IconButton
        size="xs"
        position="absolute"
        top={1}
        right={1}
        icon={<LuArrowLeftRight />}
        {...props}
      />
    </Tooltip>
  );
};

const LaunchPage = () => {
  const { t } = useTranslation();
  const router = useRouter();
  const { getSelectedPlayer, getSelectedGameInstance } = useData();
  const themedStyles = useThemedCSSStyle();

  const [selectedPlayer, setSelectedPlayer] = useState<Player>();
  const [selectedGameInstance, setSelectedGameInstance] =
    useState<GameInstanceSummary>();

  useEffect(() => {
    setSelectedPlayer(getSelectedPlayer());
  }, [getSelectedPlayer]);

  useEffect(() => {
    setSelectedGameInstance(getSelectedGameInstance());
  }, [getSelectedGameInstance]);

  return (
    <HStack position="absolute" bottom={7} right={7} spacing={4}>
      <Card
        className={
          styles["selected-user-card"] + " " + themedStyles.card["card-back"]
        }
      >
        <SwitchButton
          tooltip={t("LaunchPage.SwitchButton.tooltip.switchPlayer")}
          aria-label="switch-player"
          variant="subtle"
          onClick={() => router.push("/accounts")}
        />
        <HStack spacing={2.5} h="100%" w="100%">
          {selectedPlayer ? (
            <>
              <Image
                boxSize="32px"
                objectFit="cover"
                src={base64ImgSrc(selectedPlayer.avatar)}
                alt={selectedPlayer.name}
              />
              <VStack spacing={0} align="left" mt={-2}>
                <Text
                  fontSize="xs-sm"
                  className="no-select ellipsis-text"
                  fontWeight="bold"
                  w="100%"
                  mt={2}
                >
                  {selectedPlayer.name}
                </Text>
                <Text fontSize="2xs" className="secondary-text no-select">
                  {t(
                    `Enums.playerTypes.${selectedPlayer.playerType === "3rdparty" ? "3rdpartyShort" : selectedPlayer.playerType}`
                  )}
                </Text>
                <Text fontSize="2xs" className="secondary-text no-select">
                  {selectedPlayer.playerType === "3rdparty" &&
                    selectedPlayer.authServer?.name}
                </Text>
              </VStack>
            </>
          ) : (
            <Center w="100%" h="100%">
              <Text fontSize="sm" className="secondary-text no-select">
                {t("LaunchPage.Text.noSelectedPlayer")}
              </Text>
            </Center>
          )}
        </HStack>
      </Card>
      <Box position="relative">
        <Button colorScheme="blackAlpha" className={styles["launch-button"]}>
          <VStack spacing={1.5} w="100%" color="white">
            <Text fontSize="lg" fontWeight="bold">
              {t("LaunchPage.button.launch")}
            </Text>
            <Text fontSize="sm" className="ellipsis-text">
              {selectedGameInstance
                ? selectedGameInstance.name
                : t("LaunchPage.Text.noSelectedGame")}
            </Text>
          </VStack>
        </Button>

        <SwitchButton
          tooltip={t("LaunchPage.SwitchButton.tooltip.switchGame")}
          aria-label="switch-game"
          colorScheme={useColorModeValue("blackAlpha", "gray")}
          onClick={() => router.push("/games/all")}
        />
      </Box>
    </HStack>
  );
};

export default LaunchPage;
