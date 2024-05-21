import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

TokenResponse = WS.sendRequest(findTestObject('Postman/Authorization/Obtain access token', [('auth_url') : GlobalVariable.auth_url]))
WS.verifyResponseStatusCode(TokenResponse, 200)

if (WS.getResponseStatusCode(TokenResponse)==200) {
	println("Stauts code is 200OK")
}
else {
	println("Failed to get status code"+WS.getResponseStatusCode(TokenResponse))
}
def accesstoken=WS.getElementText(TokenResponse, 'access_token')
GlobalVariable.ctp_access_token="Bearer"+" "+accesstoken

println(GlobalVariable.ctp_access_token)

def tokentype=WS.getElementPropertyValue(TokenResponse,'token_type')

    println(tokentype)
