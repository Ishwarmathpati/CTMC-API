<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>c349ed6b-f44b-4e19-b5ea-b48008866683</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>cr55lAtcrvfOEwV3QORWX8Eya_HmROMf</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;cart\&quot; : {\n    \&quot;id\&quot;: \&quot;${GlobalVariable.cart_id}\&quot;,\n    \&quot;typeId\&quot; : \&quot;cart\&quot;\n  },\n  \&quot;version\&quot; :${GlobalVariable.cart_version}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.ctp_access_token}</value>
      <webElementGuid>6bf8cdda-a79d-436a-86e3-ce5a3cd04c9c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/kv-qa-team/orders</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>13c38c0d-b9c3-452e-8246-4b3f57c1b7f7</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>'kv-qa-team'</defaultValue>
      <description></description>
      <id>f3d89d92-0265-4b87-96a3-4b9db23b1caa</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_id</defaultValue>
      <description></description>
      <id>3172027b-7a86-4230-bdce-5ad0960b705a</id>
      <masked>false</masked>
      <name>cart_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_version</defaultValue>
      <description></description>
      <id>7c55776a-001a-4ee1-b5fa-2a1c876a9976</id>
      <masked>false</masked>
      <name>cart_version</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'type', &quot;Order&quot;)
WS.verifyElementPropertyValue(response, '', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
